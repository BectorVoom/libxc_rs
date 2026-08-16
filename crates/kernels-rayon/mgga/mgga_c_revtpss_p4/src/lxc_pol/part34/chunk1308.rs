//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1308/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1308(t29502: f64, t4248: f64, t2014: f64, t22483: f64, t7934: f64, t1497: f64, t29547: f64, t77: f64, t1493: f64, t5816: f64, t22656: f64, t84: f64) -> (f64, f64, f64, f64, f64) {
    let t114230 = 12.0_f64 * t4248 * t29502;
    let t114238 = 3.0_f64 * t2014 * t7934 * t22483;
    let t114246 = t77 * t29547 * t1497;
    let t114260 = t77 * t1493 * t5816;
    let t114264 = t77 * t84 * t22656;
    (t114230, t114238, t114246, t114260, t114264)
}
