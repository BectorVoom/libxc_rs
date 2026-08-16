//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 822/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk822(t3278: f64, t3285: f64, t3289: f64, t3288: f64, t7178: f64, t1092: f64, t3402: f64, t9282: f64, t3408: f64, t612: f64, t7451: f64, t2545: f64, t7453: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9595 = t3278 * t3285;
    let t9597 = t3278 * t3289;
    let t9599 = t3288 * t7178;
    let t9600 = t1092 * t9599;
    let t9602 = t3402 * t9282;
    let t9603 = t9602 * t3408;
    let t9605 = t7451 * t612;
    let t9606 = t2545 * t7453;
    (t9595, t9597, t9599, t9600, t9603, t9605, t9606)
}
