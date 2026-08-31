//! Safe fractional power functions, backed by rmath.

// `rmath` below is `crate::rmath` -- this crate's BitExact surface, not the
// upstream crate, whose free functions are deliberately the Fast path.
use crate::rmath;

// 2^(k/3) rescaling factors, selected by `xe % 3` in {-2, -1, 0, 1, 2}.
// pub(crate) because `simd::cbrt` replicates this function lanewise and must
// use the identical constants to stay bit-identical.
pub(crate) const CBRT_F_M2: f64 = 0.629960524947436582384; // 2^(-2/3)
pub(crate) const CBRT_F_M1: f64 = 0.793700525984099737376; // 2^(-1/3)
pub(crate) const CBRT_F_P1: f64 = 1.259921049894873164767; // 2^( 1/3)
pub(crate) const CBRT_F_P2: f64 = 1.587401051968199474752; // 2^( 2/3)

/// True cube root of an `f64`, backed by `rmath::cbrt`.
#[inline(always)]
pub fn cbrt_f64(x: f64) -> f64 {
    rmath::cbrt(x)
}

/// Cube root, handling negatives correctly. libxc `CBRT(x)` / `POW_1_3(x)`.
#[inline(always)]
pub fn safe_cbrt(x: f64) -> f64 {
    rmath::cbrt(x)
}

/// x^(1/3)
#[inline(always)]
pub fn pow_1_3(x: f64) -> f64 {
    safe_cbrt(x)
}

/// x^(2/3) = cbrt(x)^2
#[inline(always)]
pub fn pow_2_3(x: f64) -> f64 {
    let c = safe_cbrt(x);
    c * c
}

/// x^(4/3) = x * cbrt(x)
#[inline(always)]
pub fn pow_4_3(x: f64) -> f64 {
    x * safe_cbrt(x)
}

/// x^(5/3) = x * cbrt(x)^2
#[inline(always)]
pub fn pow_5_3(x: f64) -> f64 {
    let c = safe_cbrt(x);
    x * c * c
}

/// x^(7/3) = x * x * cbrt(x)
#[inline(always)]
pub fn pow_7_3(x: f64) -> f64 {
    x * x * safe_cbrt(x)
}

/// x^(3/2) = x * sqrt(x)
#[inline(always)]
pub fn pow_3_2(x: f64) -> f64 {
    x * rmath::sqrt(x)
}

/// x^(1/4) = sqrt(sqrt(x))
#[inline(always)]
pub fn pow_1_4(x: f64) -> f64 {
    rmath::sqrt(rmath::sqrt(x))
}

/// x^2
#[inline(always)]
pub fn pow_2(x: f64) -> f64 {
    x * x
}

/// x^3
#[inline(always)]
pub fn pow_3(x: f64) -> f64 {
    x * x * x
}
