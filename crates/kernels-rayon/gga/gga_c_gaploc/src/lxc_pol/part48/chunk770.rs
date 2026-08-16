//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 770/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk770(t544: f64, t8261: f64, t197: f64, t3529: f64, t107: f64, t11279: f64, t11433: f64, t1397: f64, t11429: f64, t11425: f64, t1415: f64, t1: f64, t35951: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37551 = t544 * t8261;
    let t37573 = t197 * t3529;
    let t37575 = t544 * t37573 * t107;
    let t37578 = t11279 * t107;
    let t37579 = t544 * t37578;
    let t37648 = t1397 * t11433;
    let t37654 = t1397 * t11429;
    let t37667 = t1415 * t11425;
    let t37675 = t544 * t35951 * t1;
    (t37551, t37573, t37575, t37579, t37648, t37654, t37667, t37675)
}
