//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 756/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk756(t1165: f64, t1427: f64, t604: f64, t8463: f64, t1181: f64, t1416: f64, t7575: f64, t1345: f64, t7351: f64, t1350: f64, t7426: f64, t1164: f64, t2325: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8465 = t1165 * t604 * t1427;
    let t8466 = t8463 * t8465;
    let t8469 = t1181 * t604 * t1416;
    let t8470 = t7575 * t8469;
    let t8473 = t1165 * t7351 * t1345;
    let t8474 = t7575 * t8473;
    let t8476 = t604 * t1350;
    let t8477 = t1181 * t8476;
    let t8478 = t7426 * t8477;
    let t8480 = t1164 * t2325;
    (t8465, t8466, t8469, t8470, t8473, t8474, t8476, t8477, t8478, t8480)
}
