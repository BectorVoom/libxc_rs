//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 834/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk834(t6358: f64, t180: f64, t2124: f64, t2109: f64, t746: f64, t172: f64, t2018: f64, t677: f64, t10: f64, t2054: f64, t6299: f64, t138: f64, t2022: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6359 = 1.0_f64 / t6358;
    let t6363 = t180 * t2124;
    let t6383 = 1.0_f64 / t2109 / t746;
    let t6394 = 1.0_f64 / t6358 / t172;
    let t6425 = t677 * t2018;
    let t6429 = t2054 * t10;
    let t6449 = t6299 * t10;
    let t6452 = 1.0_f64 / t138 / t2022;
    (t6359, t6363, t6383, t6394, t6425, t6429, t6449, t6452)
}
