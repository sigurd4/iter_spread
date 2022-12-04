pub trait IterSpread: Iterator
{
    type Output;

    /// Spreads the iterated elements across an array
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let a = [1.0, 2.0, 3.0, 4.0];
    /// let b: [Vec<f32>; 2] = a.iter().map(|a| *a).spread_array();
    /// ```
    fn spread_array<const SIZE: usize>(self) -> [Self::Output; SIZE];

    /// Spreads the iterated elements across a vector
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let a = [1.0, 2.0, 3.0, 4.0];
    /// let b: Vec<Vec<f32>> = a.iter().map(|a| *a).spread_vec(2);
    /// ```
    fn spread_vec(self, size: usize) -> Vec<Self::Output>;
}

impl<T, I> IterSpread for I
where
    I: Iterator<Item=T>
{
    type Output = Vec<T>;
    fn spread_array<const SIZE: usize>(self) -> [Self::Output; SIZE]
    {
        use array_init::array_init;
        let mut y: [Vec<T>; SIZE] = array_init(|_| vec![]);
        let mut i = (0..SIZE).cycle();
        self.for_each(|xn| y[i.next().unwrap()].push(xn));
        y
    }
    fn spread_vec(self, size: usize) -> Vec<Self::Output>
    {
        let mut y: Vec<Vec<T>> = (0..size).map(|_| vec![]).collect();
        let mut i = (0..size).cycle();
        self.for_each(|xn| y[i.next().unwrap()].push(xn));
        y
    }
}

#[cfg(test)]
mod tests {
    use crate::IterSpread;

    #[test]
    fn it_works() {
        let a = [1.0, 2.0, 3.0, 4.0];
        let _b: [Vec<f32>; 2] = a.iter().map(|a| *a).spread_array();
    }
}
