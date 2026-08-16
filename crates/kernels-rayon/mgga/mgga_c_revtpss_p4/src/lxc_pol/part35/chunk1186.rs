//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1186/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1186(t23429: f64, t33: f64, t1544: f64, t6416: f64, t113107: f64, t27799: f64, t1497: f64, t29547: f64, t77: f64, t1493: f64, t5816: f64, t22656: f64, t84: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t114188 = t33 * t23429;
    let t114192 = t6416 * t1544;
    let t114196 = t27799 * t113107;
    let t114246 = t77 * t29547 * t1497;
    let t114260 = t77 * t1493 * t5816;
    let t114264 = t77 * t84 * t22656;
    (t114188, t114192, t114196, t114246, t114260, t114264)
}
