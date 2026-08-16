//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta83 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk484;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk485;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk486;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk487;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta83(t1659: f64, t366: f64, t1651: f64, t373: f64, t372: f64, t371: f64, t1598: f64, t1612: f64, t1638: f64, t1640: f64, t1644: f64, t1045: f64, t1042: f64, t1066: f64, t1592: f64, t247: f64, t1009: f64, t1011: f64, t1025: f64, t1041: f64, t1060: f64, t1063: f64, t1656: f64, t375: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1660, t1663, t1665, t1668) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk484(t1659, t366, t1651, t373, t372, t371, t1598, t1612, t1638, t1640, t1644);
        let (t1670, t1671) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk485(t1668, t373, t1045, t1042);
        let t1675 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk486(t1066, t1592, t247);
        let t1678 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk487(t1009, t1011, t1025, t1041, t1060, t1063, t1656, t1660, t1665, t1671, t1675, t375);
    (t1660, t1663, t1665, t1668, t1670, t1671, t1675, t1678)
}
