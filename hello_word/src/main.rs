fn main() {
    let x = vec![1, 2, 3];

    let x = print_vector(x);

    println!("{:?}", x);
}

fn print_vector(v: Vec<i32>) -> Vec<i32> {
   println!("I took this data: {:?}, and returned it", v);
   v
}