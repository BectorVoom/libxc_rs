//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta615 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2149;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2150;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2151;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta615<F: Float>(t18875: F, t92790: F, t1468: F, t2832: F, t2408: F, t25207: F, t61182: F, t2430: F, t1583: F, t2257: F, t2394: F, t11064: F, t605: F, t27384: F, t27375: F, t890: F, t27383: F, t1940: F, t1963: F, t2403: F, t25206: F, t25211: F, t25440: F, t25445: F, t27158: F, t27166: F, t27364: F, t27382: F, t27387: F, t7010: F, t7091: F, t7783: F, t7787: F, t92775: F, t92819: F, t9342: F, t30: F, t41154: F, t1957: F, t25392: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t98733, t98736, t98740, t98743, t98751, t98755, t98759, t98760, t98763) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2149::<F>(t18875, t92790, t1468, t2832, t2408, t25207, t61182, t2430, t1583, t2257, t2394, t11064, t605);
        let (t98767, t98776) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2150::<F>(t27384, t98763, t27375, t890, t27383, t1940, t1963, t2257, t2403, t25206, t25211, t25440, t25445, t27158, t27166, t27364, t27382, t27387, t7010, t7091, t7783, t7787, t92775, t92819, t98733, t98736, t98740, t98743, t98751, t98755, t98760);
        let (t98779, t98780, t98784, t98786, t98787, t98793, t98799) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2151::<F>(t1583, t2832, t27383, t1940, t1963, t9342, t30, t41154, t2408, t1468, t2394, t1957, t25392);
    (t98759, t98767, t98776, t98779, t98780, t98784, t98786, t98787, t98793, t98799)
}
