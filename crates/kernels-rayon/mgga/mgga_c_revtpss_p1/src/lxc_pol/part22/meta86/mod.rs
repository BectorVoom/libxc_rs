//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta86 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk617;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk618;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk619;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk620;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk621;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk622;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk623;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk624;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk625;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta86(t1802: f64, t475: f64, t467: f64, t1264: f64, t1715: f64, t247: f64, t1221: f64, t1222: f64, t1235: f64, t1247: f64, t1258: f64, t1261: f64, t1778: f64, t1782: f64, t1786: f64, t1791: f64, t1797: f64, t464: f64, t484: f64, t225: f64, t494: f64, t1280: f64, t1774: f64, t1287: f64, t1794: f64, t487: f64, t489: f64, t1234: f64, t1285: f64, t1770: f64, t460: f64, t490: f64, t1277: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1803 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk617(t1802, t475);
        let (t1804, t1808) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk618(t1803, t467, t1264, t1715, t247);
        let t1811 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk619(t1221, t1222, t1235, t1247, t1258, t1261, t1778, t1782, t1786, t1791, t1797, t1804, t1808, t464, t484);
        let (t1812, t1813) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk620(t1811, t225, t494);
        let t1818 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk621(t1280, t1774);
        let t1822 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk622(t1287, t1794, t487);
        let t1825 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk623(t1811, t489);
        let t1828 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk624(t1234, t1285, t1770, t1818, t1822, t1825, t460, t490);
        let t1829 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk625(t1277, t1828);
    (t1803, t1804, t1808, t1811, t1812, t1813, t1818, t1822, t1825, t1828, t1829)
}
