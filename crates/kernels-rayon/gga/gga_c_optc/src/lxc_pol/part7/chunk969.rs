//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 969/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk969(t9279: f64, t9291: f64, t1199: f64, t2879: f64, t1196: f64, t2885: f64, t1198: f64, t481: f64, t1205: f64, t2887: f64, t2900: f64, t8639: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9292 = t9279 + t9291;
    let t9294 = t2879 * t1199;
    let t9297 = t1196 * t2885;
    let t9302 = t1198 * t1198;
    let t9303 = 1.0_f64 / t9302;
    let t9304 = t481 * t9303;
    let t9305 = t2887 * t1205;
    let t9308 = t1205 * t2900;
    let t9311 = 0.22615185185185185185e4_f64 * t8639;
    (t9292, t9294, t9297, t9302, t9303, t9304, t9305, t9308, t9311)
}
