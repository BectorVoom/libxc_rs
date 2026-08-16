//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta574 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1757;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1758;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta574<F: Float>(t90437: F, t90449: F, t1139: F, t43821: F, t90422: F, t43814: F, t43817: F, t89824: F, t89832: F, t90402: F, t90405: F, t90408: F, t90411: F, t90414: F, t90417: F, t90420: F, t90423: F, t1132: F, t3407: F, t90419: F, t141: F, t3417: F, t89841: F, t89826: F, t81230: F, t81232: F, t81234: F, t81425: F, t81427: F, t81429: F, t89828: F, t89843: F, t89847: F, t89855: F) -> (F, F, F, F, F, F, F, F) {
        let (t90450, t90451, t90453, t90456) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1757::<F>(t90437, t90449, t1139, t43821, t90422, t43814, t43817, t89824, t89832, t90402, t90405, t90408, t90411, t90414, t90417, t90420, t90423);
        let (t90459, t90464, t90470, t90473, t90478) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1758::<F>(t1132, t90450, t3407, t90419, t141, t3417, t89841, t89826, t81230, t81232, t81234, t81425, t81427, t81429, t89828, t89843, t89847, t89855);
    (t90451, t90453, t90456, t90459, t90464, t90470, t90473, t90478)
}
