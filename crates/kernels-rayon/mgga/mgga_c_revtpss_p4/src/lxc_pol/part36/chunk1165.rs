//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1165/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1165(t5868: f64, t76: f64, t1470: f64, t4173: f64, t1493: f64, t1497: f64, t77: f64, t5872: f64, t84: f64, t5819: f64, t603: f64, t5826: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29532 = t76 * t5868;
    let t29538 = t4173 * t1470;
    let t29543 = t1493 * t1497;
    let t29544 = t77 * t29543;
    let t29547 = t84 * t5872;
    let t29548 = t77 * t29547;
    let t29551 = t603 * t5819;
    let t29554 = t603 * t5826;
    (t29532, t29538, t29544, t29547, t29548, t29551, t29554)
}
