//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta621 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2134;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2135;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2136;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta621(t18875: f64, t92790: f64, t1468: f64, t2832: f64, t2408: f64, t25207: f64, t61182: f64, t2430: f64, t1583: f64, t2257: f64, t2394: f64, t11064: f64, t605: f64, t27384: f64, t27375: f64, t890: f64, t27383: f64, t1940: f64, t1963: f64, t2403: f64, t25206: f64, t25211: f64, t25440: f64, t25445: f64, t27158: f64, t27166: f64, t27364: f64, t27382: f64, t27387: f64, t7010: f64, t7091: f64, t7783: f64, t7787: f64, t92775: f64, t92819: f64, t9342: f64, t30: f64, t41154: f64, t1957: f64, t25392: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98733, t98736, t98740, t98743, t98751, t98755, t98759, t98760, t98763) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2134(t18875, t92790, t1468, t2832, t2408, t25207, t61182, t2430, t1583, t2257, t2394, t11064, t605);
        let (t98767, t98776) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2135(t27384, t98763, t27375, t890, t27383, t1940, t1963, t2257, t2403, t25206, t25211, t25440, t25445, t27158, t27166, t27364, t27382, t27387, t7010, t7091, t7783, t7787, t92775, t92819, t98733, t98736, t98740, t98743, t98751, t98755, t98760);
        let (t98779, t98780, t98784, t98786, t98787, t98793, t98799) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2136(t1583, t2832, t27383, t1940, t1963, t9342, t30, t41154, t2408, t1468, t2394, t1957, t25392);
    (t98759, t98767, t98776, t98779, t98780, t98784, t98786, t98787, t98793, t98799)
}
