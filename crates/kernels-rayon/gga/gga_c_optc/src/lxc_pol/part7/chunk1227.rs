//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1227/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1227(t11450: f64, t8193: f64, t11454: f64, t2606: f64, t857: f64, t3917: f64, t3918: f64, t3884: f64, t3886: f64, t2723: f64, t7257: f64, t10959: f64, t2812: f64, t8164: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25335 = t11450 * t8193;
    let t25338 = t11454 * t8193;
    let t25341 = t857 * t2606;
    let t25343 = t3917 * t25341 * t3918;
    let t25346 = t3884 * t25341 * t3886;
    let t25348 = t2723 * t7257;
    let t25353 = t2812 * t10959 * t8164;
    (t25335, t25338, t25343, t25346, t25348, t25353)
}
