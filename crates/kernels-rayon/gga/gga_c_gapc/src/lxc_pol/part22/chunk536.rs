//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 536/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk536(t3084: f64, t3085: f64, t129: f64, t1932: f64, t1023: f64, t1928: f64, t197: f64, t1022: f64, t1018: f64, t611: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3086 = t3084 * t3085;
    let t3088 = t1932 * t129;
    let t3089 = t3088 * t1023;
    let t3091 = t197 * t1928;
    let t3092 = t1022 * t3091;
    let t3094 = t611 * t1018;
    (t3086, t3088, t3089, t3091, t3092, t3094)
}
