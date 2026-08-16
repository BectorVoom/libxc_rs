//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1974/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1974(t101398: f64, t101413: f64, t101425: f64, t101439: f64, t101456: f64, t101468: f64, t101486: f64, t101496: f64, t1528: f64, t17056: f64, t218: f64, t25168: f64, t259: f64, t26728: f64, t2713: f64, t29091: f64, t86983: f64, t86991: f64, t86994: f64, t92386: f64, t98251: f64, t98256: f64, t98264: f64, t98277: f64) -> (f64, f64) {
    let t101499 = t101398 + t101413 + t101425 + t101439 + t101456 + t101468 + t101486 + t101496;
    let t101504 = -6.0_f64 * t2713 * t29091 + 0.6579736267392905746e-1_f64 * t98251 + 0.3289868133696452873e-1_f64 * t98256 + 0.6579736267392905746e-1_f64 * t98264 - 2.0_f64 * t92386 * t1528 + t86983 - 6.0_f64 * t25168 * t26728 * t17056 + t218 * t101499 * t259 - 0.13159472534785811492e0_f64 * t98277 - 0.25587863262083522345e0_f64 * t86991 + t86994;
    (t101499, t101504)
}
