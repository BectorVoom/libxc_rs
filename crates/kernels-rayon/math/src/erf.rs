//! Error function (erf) and complementary error function (erfc) implementations backed by rmath.

pub use crate::special::xc_erfcx;

/// Compute the error function erf(x) with full f64 precision.
#[inline(always)]
pub fn erf_approx(x: f64) -> f64 {
    rmath::erf(x)
}

/// Backward-compatible alias for generated kernels that still reference the
/// older CubeCL-facing helper name.
#[inline(always)]
pub fn erf_cube(x: f64) -> f64 {
    rmath::erf(x)
}

/// Compute the complementary error function erfc(x) = 1 - erf(x).
#[inline(always)]
pub fn erfc_approx(x: f64) -> f64 {
    rmath::erfc(x)
}
