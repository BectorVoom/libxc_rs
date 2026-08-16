//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 857/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk857(t1234: f64, t3096: f64, t1233: f64, t18091: f64, t18089: f64, t18096: f64, t92: f64, t135: f64, t9105: f64, t4082: f64, t4085: f64, t1247: f64, t2282: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39635 = 1.0_f64 / t1234 / t3096;
    let t39636 = t1233 * t39635;
    let t39637 = t39636 * t18091;
    let t39642 = t18096 * t1233 * t39635 * t18089 * t92;
    let t39644 = t135 * t9105;
    let t39646 = t4082 * t39644 * t4085;
    let t39648 = t1247 * t2282;
    (t39635, t39636, t39637, t39642, t39644, t39646, t39648)
}
