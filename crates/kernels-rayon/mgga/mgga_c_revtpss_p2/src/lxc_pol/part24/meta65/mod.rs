//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta65 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk412;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk413;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta65(t1015: f64, t1469: f64, t1012: f64, t1647: f64, t225: f64, t366: f64, t1651: f64, t373: f64, t372: f64, t371: f64, t1598: f64, t1612: f64, t1638: f64, t1640: f64, t1644: f64, t1045: f64, t1042: f64, t1066: f64, t1592: f64, t247: f64, t1009: f64, t1011: f64, t1025: f64, t1041: f64, t1060: f64, t1063: f64, t375: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1655, t1656, t1659, t1660, t1663, t1665, t1668) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk412(t1015, t1469, t1012, t1647, t225, t366, t1651, t373, t372, t371, t1598, t1612, t1638, t1640, t1644);
        let (t1670, t1671, t1675, t1678) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk413(t1668, t373, t1045, t1042, t1066, t1592, t247, t1009, t1011, t1025, t1041, t1060, t1063, t1656, t1660, t1665, t375);
    (t1655, t1659, t1660, t1663, t1665, t1668, t1670, t1671, t1675, t1678)
}
