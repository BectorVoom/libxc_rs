//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1077/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1077(t12018: f64, t2268: f64, t894: f64, t13756: f64, t419: f64, t13751: f64, t380: f64, t42625: f64, t42629: f64, t42633: f64, t42637: f64, t42638: f64, t42641: f64, t42645: f64, t42648: f64) -> f64 {
    let t46928 = t2268 * t894 * t12018;
    let t46931 = 0.28455006635676149599e-1_f64 * t419 * t13756;
    let t46933 = 0.37940008847568199465e-1_f64 * t380 * t13751;
    let t46935 = 0.28455006635676149599e-1_f64 * t46928 + t46931 - t46933 - 0.56910013271352299198e-1_f64 * t42625 - t42629 - t42633 + t42637 - t42638 + t42641 - t42645 + t42648;
    t46935
}
