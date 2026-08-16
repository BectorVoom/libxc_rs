//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1162/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1162(t2057: f64, t6012: f64, t17: f64, t2022: f64, t697: f64, t2053: f64, t140: f64, t19746: f64, t35: f64, t2062: f64, t150: f64, t168: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20282 = t6012 * t2057;
    let t20290 = t17 / t697 / t2022;
    let t20291 = t2053 * t2053;
    let t20292 = 1.0_f64 / t20291;
    let t20346 = 140.0_f64 / 729.0_f64 * t35 * t19746 * t140;
    let t20355 = t6012 * t2062;
    let t20396 = 1.0_f64 / t168 / t150;
    (t20282, t20290, t20292, t20346, t20355, t20396)
}
