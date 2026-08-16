//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 847/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk847(t2669: f64, t7373: f64, t2679: f64, t2606: f64, t8002: f64, t8108: f64, t2746: f64, t298: f64, t301: f64, t305: f64, t8113: f64, t19: f64, t7380: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8115 = t2669 * t7373;
    let t8116 = t8115 * t2679;
    let t8119 = t8002 * t2606;
    let t8120 = t8108 * t8119;
    let t8124 = 1.0_f64 / t2746 / t298;
    let t8125 = t8124 * t301;
    let t8126 = t8125 * t305;
    let t8127 = t8126 * t8113;
    let t8128 = t7380 * t19;
    (t8115, t8116, t8120, t8124, t8125, t8126, t8127, t8128)
}
