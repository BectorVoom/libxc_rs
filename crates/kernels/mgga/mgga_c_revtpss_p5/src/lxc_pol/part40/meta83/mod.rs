//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta83 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk484;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk485;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk486;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk487;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta83<F: Float>(t1659: F, t366: F, t1651: F, t373: F, t372: F, t371: F, t1598: F, t1612: F, t1638: F, t1640: F, t1644: F, t1045: F, t1042: F, t1066: F, t1592: F, t247: F, t1009: F, t1011: F, t1025: F, t1041: F, t1060: F, t1063: F, t1656: F, t375: F) -> (F, F, F, F, F, F, F, F) {
        let (t1660, t1663, t1665, t1668) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk484::<F>(t1659, t366, t1651, t373, t372, t371, t1598, t1612, t1638, t1640, t1644);
        let (t1670, t1671) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk485::<F>(t1668, t373, t1045, t1042);
        let t1675 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk486::<F>(t1066, t1592, t247);
        let t1678 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk487::<F>(t1009, t1011, t1025, t1041, t1060, t1063, t1656, t1660, t1665, t1671, t1675, t375);
    (t1660, t1663, t1665, t1668, t1670, t1671, t1675, t1678)
}
