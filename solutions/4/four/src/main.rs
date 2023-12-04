use std::{fs, io::Split};

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let mut sum = 0;
    for line in contents.lines() {
        let it = line.split("|");
        let result = calculate_points(it);
        sum = sum + result;
        break;
    }
}


fn calculate_points(line: Split<str>) -> i32 {
   0 
}
