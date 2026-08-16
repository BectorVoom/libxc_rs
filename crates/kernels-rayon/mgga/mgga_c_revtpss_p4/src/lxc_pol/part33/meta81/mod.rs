//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta81 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk521;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk522;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk523;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk524;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk525;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk526;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta81(t1668: f64, t373: f64, t1045: f64, t1042: f64, t1066: f64, t1592: f64, t247: f64, t1009: f64, t1011: f64, t1025: f64, t1041: f64, t1060: f64, t1063: f64, t1656: f64, t1660: f64, t1665: f64, t375: f64, t225: f64, t385: f64, t1082: f64, t1651: f64, t1089: f64, t378: f64, t380: f64, t1024: f64, t1087: f64, t1647: f64, t342: f64, t381: f64, t1079: f64, t265: f64, t393: f64, t1076: f64, t1652: f64, t386: f64, t995: f64, t1102: f64, t1587: f64, t1598: f64, t1612: f64, t1638: f64, t1640: f64, t1644: f64, t198: f64, t336: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1670, t1671) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk521(t1668, t373, t1045, t1042);
        let t1675 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk522(t1066, t1592, t247);
        let t1678 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk523(t1009, t1011, t1025, t1041, t1060, t1063, t1656, t1660, t1665, t1671, t1675, t375);
        let (t1680, t1685, t1689, t1692, t1695) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk524(t1678, t225, t385, t1082, t1651, t1089, t1668, t378, t380, t1024, t1087, t1647, t342, t381);
        let t1696 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk525(t1079, t1695);
        let (t1699, t1704) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk526(t265, t393, t1076, t1647, t1652, t1680, t1696, t342, t386, t995, t1102, t1587, t1598, t1612, t1638, t1640, t1644, t198, t336);
    (t1670, t1671, t1675, t1678, t1680, t1685, t1689, t1692, t1695, t1696, t1699, t1704)
}
