//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1934/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1934(t1493: f64, t1497: f64, t77: f64, t5872: f64, t84: f64, t5819: f64, t603: f64, t5826: f64, t5816: f64, t2034: f64, t22475: f64, t2014: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29543 = t1493 * t1497;
    let t29544 = t77 * t29543;
    let t29547 = t84 * t5872;
    let t29548 = t77 * t29547;
    let t29551 = t603 * t5819;
    let t29554 = t603 * t5826;
    let t29561 = t84 * t5816;
    let t29562 = t77 * t29561;
    let t29576 = t2034 * t22475;
    let t29578 = 2.0_f64 * t2014 * t29576;
    (t29543, t29544, t29547, t29548, t29551, t29554, t29561, t29562, t29576, t29578)
}
