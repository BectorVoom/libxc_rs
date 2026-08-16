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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta86<F: Float>(t1802: F, t475: F, t467: F, t1264: F, t1715: F, t247: F, t1221: F, t1222: F, t1235: F, t1247: F, t1258: F, t1261: F, t1778: F, t1782: F, t1786: F, t1791: F, t1797: F, t464: F, t484: F, t225: F, t494: F, t1280: F, t1774: F, t1287: F, t1794: F, t487: F, t489: F, t1234: F, t1285: F, t1770: F, t460: F, t490: F, t1277: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t1803 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk617::<F>(t1802, t475);
        let (t1804, t1808) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk618::<F>(t1803, t467, t1264, t1715, t247);
        let t1811 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk619::<F>(t1221, t1222, t1235, t1247, t1258, t1261, t1778, t1782, t1786, t1791, t1797, t1804, t1808, t464, t484);
        let (t1812, t1813) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk620::<F>(t1811, t225, t494);
        let t1818 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk621::<F>(t1280, t1774);
        let t1822 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk622::<F>(t1287, t1794, t487);
        let t1825 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk623::<F>(t1811, t489);
        let t1828 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk624::<F>(t1234, t1285, t1770, t1818, t1822, t1825, t460, t490);
        let t1829 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk625::<F>(t1277, t1828);
    (t1803, t1804, t1808, t1811, t1812, t1813, t1818, t1822, t1825, t1828, t1829)
}
