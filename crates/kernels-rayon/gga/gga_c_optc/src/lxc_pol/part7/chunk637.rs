//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 637/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk637(t1179: f64, t3141: f64, t462: f64, t442: f64, t2665: f64, t446: f64, t140: f64, t3105: f64, t3107: f64, t1: f64, t450: f64, t3101: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3175 = t1179 * t3141;
    let t3181 = t462 * t462;
    let t3182 = 1.0_f64 / t3181;
    let t3183 = t3182 * t442;
    let t3184 = t446 * t2665;
    let t3185 = t3184 * t140;
    let t3186 = t3183 * t3185;
    let t3187 = t3105 * t3107;
    let t3188 = t3187 * t1;
    let t3189 = t450 * t3188;
    let t3192 = t3101 * t3185;
    (t3175, t3181, t3182, t3183, t3186, t3187, t3188, t3189, t3192)
}
