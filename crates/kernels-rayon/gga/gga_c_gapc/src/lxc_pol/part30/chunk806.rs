//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 806/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk806(t1092: f64, t9599: f64, t3402: f64, t9282: f64, t3408: f64, t612: f64, t7451: f64, t2545: f64, t7453: f64, t197: f64, t7776: f64, t1077: f64) -> (f64, f64, f64, f64) {
    let t9600 = t1092 * t9599;
    let t9602 = t3402 * t9282;
    let t9603 = t9602 * t3408;
    let t9605 = t7451 * t612;
    let t9606 = t2545 * t7453;
    let t9607 = t9605 * t9606;
    let t9609 = t197 * t7776;
    let t9610 = t1077 * t9609;
    (t9600, t9603, t9607, t9610)
}
