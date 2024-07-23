use macroquad::prelude::*;
extern crate rand;
use rand::prelude::*;
use ndarray::prelude::*;
use std::cmp::{min, max};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

fn main() {
    let num_maps = 50;
    for i in 0..num_maps {
	let map_size = 513;
	let mut map = Array::<usize, Ix2>::ones((map_size, map_size));

	dig(1, 1, &mut map);
	split(&mut map);

	let file_name = format!("maps_{}x{}/grid_{}.csv", map_size, map_size, i);
	let file = File::create(file_name).expect("File creation failed");
	let mut buf = BufWriter::new(file);
	for i in  0..map.len_of(Axis(0)) {
	    for j in 0..map.len_of(Axis(1)) {
		let data = format!("{}, ", map[[i,j]]);
		buf.write(data.as_bytes()).unwrap();
	    }
	    buf.write("\n".as_bytes()).unwrap();
	}
    }
}

// fn find_start_goal(map: &mut Array2<usize>) {
//     'outer: for i in 0..map.len_of(Axis(0)) {
// 	for j in 0..map.len_of(Axis(1)) {
// 	    if map[[i,j]] == 0 {
// 		map[[i,j]] = 2;
// 		break 'outer;
// 	    }	    
// 	}
//     }
//     'outer: for i in (0..map.len_of(Axis(0))).rev() {
// 	for j in (0..map.len_of(Axis(1))).rev() {
// 	    if map[[i,j]] == 0 {
// 		map[[i,j]] = 3;
// 		break 'outer;
// 	    }
// 	}
//     }
// }

fn dig(ci: usize, cj: usize, map: &mut Array2<usize>) {
    let map_size = map.len_of(Axis(0)) as i32;
    let cx = cj as i32;
    let cy = ci as i32;
    let dx: Vec<i32> = vec![-5 - 1, 0, 0, 5 + 1];
    let dy: Vec<i32> = vec![0, -5 - 1, 5 + 1, 0];
    let mut neighbors = dx.iter()
	.zip(dy)
	.map(|(dx, dy)| (cx + dx, cy + dy))
	.filter(|(x, y)| *x > -1 && *x + 5 < map_size && *y > -1 && *y + 5 < map_size)
	.collect::<Vec<(i32, i32)>>();
    neighbors.as_mut_slice().shuffle(&mut thread_rng());

    for (nx, ny) in neighbors {
	let neighbor = map.slice(s![ny..ny+5, nx..nx+5]);
	let bx = (min(cx, nx) + max(cx + 5, nx + 5)) / 2;
	let by = (min(cy, ny) + max(cy + 5, ny + 5)) / 2;
	if neighbor.sum() == 5 * 5 {
	    if (nx - cx).abs() > 0 {
		map.slice_mut(s![by-2..by+3, bx]).fill(0);
	    }
	    if (ny - cy).abs() > 0 {
		map.slice_mut(s![by, bx-2..bx+3]).fill(0);
	    }
	    map.slice_mut(s![ny..ny+5, nx..nx+5]).fill(0);

	    dig(ny as usize, nx as usize, map);
	}
    }
}

fn split(map: &mut Array2<usize>) {
    let map_size = map.len_of(Axis(0)) as i32;
    let ranges = go_split(0, map_size, 0, map_size);
    let mut rng = thread_rng();
    for (xs, xe, ys, ye) in ranges.iter() {
	let ys_offset = rng.gen_range(2..10);
	let ye_offset = rng.gen_range(-10..-2);
	let xs_offset = rng.gen_range(2..10);
	let xe_offset = rng.gen_range(-10..-2);
	let ys = ys + ys_offset;
	let ye = ye + ye_offset;
	let xs = xs + xs_offset;
	let xe = xe + xe_offset;
	for i in ys..ye {
	    for j in xs..xe {
		map[[i as usize,j as usize]] = 0;
	    }
	}
    }
}

fn go_split(xs: i32, xe: i32, ys: i32, ye: i32) -> Vec<(i32, i32, i32, i32)> {
    let mut closed = Vec::new();
    let mut open = vec![(xs, xe, ys, ye)];
    let mut rng = thread_rng();
    while !open.is_empty() {
	let (xs, xe, ys, ye) = open.pop().unwrap();

	if (xe - xs) * (ye - ys) < 2000 + rng.gen_range(-500..2000) {
	    closed.push((xs, xe, ys, ye));
	    continue
	}

	let (s1, s2) = {
	    let random = rng.gen_range(-4..4);
	    if xe - xs > ye - ys {
		let partition = (xe - xs) / 2 + random + xs;
		((xs, partition, ys, ye), (partition + 1, xe, ys, ye))
	    }
	    else {
		let partition = (ye - ys) / 2 + random + ys;
		((xs, xe, ys, partition), (xs, xe, partition + 1, ye))
	    }
	};
	open.push(s1);
	open.push(s2);
    }
    closed
}
