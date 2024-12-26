fn main() {
    let mut a: u8 = 11;
    a = 33;
    let b: i8 = 12;
    let c: f32 = 0.22;
    let d = "cd";
    let e = "\u{1F600}";
    println!("Hello ${}, world!${}, {}, {}, {}", a, b, c, d, e);

    let arr: [u8; 3] = [1, 2, 3];
    let arr1: [u8; 5] = [100; 5];
    println!("{:?}", arr1);

    let tuple: (u8, bool, f32) = (1, true, 9.3);
    println!("{}", tuple.2);
}
