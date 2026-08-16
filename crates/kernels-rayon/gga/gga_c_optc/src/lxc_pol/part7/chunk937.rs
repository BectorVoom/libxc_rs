//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 937/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk937(t4289: f64, t8498: f64, t1114: f64, t6554: f64, t322: f64, t1115: f64, t530: f64, t1111: f64, t24: f64, t3097: f64, t2586: f64, t3147: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8933 = t4289 * t8498;
    let t8936 = t1114 * t6554;
    let t8937 = t322 * t8936;
    let t8940 = t530 * t1115;
    let t8941 = t1111 * t8940;
    let t8943 = t24 * t3097;
    let t8944 = t1111 * t8943;
    let t8946 = t2586 * t3147;
    (t8933, t8936, t8937, t8941, t8944, t8946)
}
