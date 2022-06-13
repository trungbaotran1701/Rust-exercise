fn main() {
    //Bai1
    fn check_sublist<T: Eq>(a: &[T], b: &[T]){
        match (a.len(), b.len()) {
            (c,d) if c < d => if b.windows(c).any(|v|v == a) 
                                            {println!("a la sublist cua b")}, 
            (c, d) if c > d => if a.windows(d).any(|v| v == b)
                                            {println!("b la sublist cua a")},
            (_,_) => println!("khong ton tai sublist trong 2 array tren"),
        }
    }

    //Bai2
    fn count(input: String, str_slice: &str ) {
        let count = str_slice.matches(&input.as_str().trim()).count();
        println!("Co {} trong string slice: {}", count, str_slice  );
    }
}
