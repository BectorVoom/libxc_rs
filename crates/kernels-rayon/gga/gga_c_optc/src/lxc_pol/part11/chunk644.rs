//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 644/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk644(t5274: f64, t285: f64, t442: f64, t5255: f64, t441: f64, t1114: f64, t4573: f64, sigma2: f64) -> (f64, f64, f64, f64, f64) {
    let t5275 = sigma2 * t5274;
    let t5276 = t5275 * t285;
    let t5279 = t442 * t5255;
    let t5280 = t441 * t5279;
    let t5285 = t1114 * t4573;
    (t5275, t5276, t5279, t5280, t5285)
}
