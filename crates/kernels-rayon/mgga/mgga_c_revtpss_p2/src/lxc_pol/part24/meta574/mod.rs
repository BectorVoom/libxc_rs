//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta574 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1757;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1758;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta574(t90437: f64, t90449: f64, t1139: f64, t43821: f64, t90422: f64, t43814: f64, t43817: f64, t89824: f64, t89832: f64, t90402: f64, t90405: f64, t90408: f64, t90411: f64, t90414: f64, t90417: f64, t90420: f64, t90423: f64, t1132: f64, t3407: f64, t90419: f64, t141: f64, t3417: f64, t89841: f64, t89826: f64, t81230: f64, t81232: f64, t81234: f64, t81425: f64, t81427: f64, t81429: f64, t89828: f64, t89843: f64, t89847: f64, t89855: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90450, t90451, t90453, t90456) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1757(t90437, t90449, t1139, t43821, t90422, t43814, t43817, t89824, t89832, t90402, t90405, t90408, t90411, t90414, t90417, t90420, t90423);
        let (t90459, t90464, t90470, t90473, t90478) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1758(t1132, t90450, t3407, t90419, t141, t3417, t89841, t89826, t81230, t81232, t81234, t81425, t81427, t81429, t89828, t89843, t89847, t89855);
    (t90451, t90453, t90456, t90459, t90464, t90470, t90473, t90478)
}
