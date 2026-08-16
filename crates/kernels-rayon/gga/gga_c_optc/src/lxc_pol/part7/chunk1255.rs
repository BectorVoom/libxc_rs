//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1255/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1255(t25035: f64, t953: f64, t140: f64, t2246: f64, t2665: f64, t2748: f64, t2661: f64, t2708: f64, t8240: f64, t2746: f64, t301: f64, t327: f64) -> (f64, f64, f64, f64, f64) {
    let t25920 = t953 * t25035;
    let t25928 = t2246 * t2665 * t140;
    let t25929 = t2748 * t25928;
    let t25932 = t2661 * t25928;
    let t25935 = t2708 * t8240;
    let t25939 = 1.0_f64 / t2746 / t327 * t301;
    (t25920, t25929, t25932, t25935, t25939)
}
