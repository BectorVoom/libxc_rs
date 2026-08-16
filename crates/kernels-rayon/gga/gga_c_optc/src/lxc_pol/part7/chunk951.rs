//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 951/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk951(t3245: f64, t9058: f64, t3105: f64, t3117: f64, t3120: f64, t3235: f64, t2850: f64, t3236: f64, t4387: f64, t1: f64, t3107: f64, t1028: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9059 = t3245 * t9058;
    let t9062 = t3117 * t3105;
    let t9063 = t9062 * t3120;
    let t9066 = t3235 * t9058;
    let t9069 = t2850 * t3236;
    let t9070 = t4387 * t9069;
    let t9073 = t3107 * t1;
    let t9074 = t9073 * t1028;
    (t9059, t9062, t9063, t9066, t9069, t9070, t9073, t9074)
}
