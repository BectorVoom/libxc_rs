//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta87 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk546;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk547;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk548;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk549;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk550;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk551;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk552;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta87(t1803: f64, t467: f64, t1264: f64, t1715: f64, t247: f64, t1221: f64, t1222: f64, t1235: f64, t1247: f64, t1258: f64, t1261: f64, t1778: f64, t1782: f64, t1786: f64, t1791: f64, t1797: f64, t464: f64, t484: f64, t225: f64, t494: f64, t1280: f64, t1774: f64, t1287: f64, t1794: f64, t487: f64, t489: f64, t1234: f64, t1285: f64, t1770: f64, t460: f64, t490: f64, t1277: f64, t1210: f64, t1274: f64, t1775: f64, t495: f64, t33: f64, t265: f64, t502: f64, t1300: f64, t1587: f64, t1721: f64, t1735: f64, t1761: f64, t1763: f64, t1767: f64, t198: f64, t336: f64, t1469: f64, t1711: f64, t504: f64, t57: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t1709: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1804, t1808) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk546(t1803, t467, t1264, t1715, t247);
        let t1811 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk547(t1221, t1222, t1235, t1247, t1258, t1261, t1778, t1782, t1786, t1791, t1797, t1804, t1808, t464, t484);
        let (t1813, t1818, t1822, t1825, t1828) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk548(t1811, t225, t494, t1280, t1774, t1287, t1794, t487, t489, t1234, t1285, t1770, t460, t490);
        let t1829 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk549(t1277, t1828);
        let t1832 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk550(t1210, t1274, t1770, t1775, t1813, t1829, t460, t495);
        let (t1837, t1842) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk551(t33, t265, t502, t1300, t1587, t1721, t1735, t1761, t1763, t1767, t1832, t198, t336, t1469, t1711, t504, t57, dens_threshold, rho1, zeta_threshold);
        let t1843 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk552(t1709, t1842);
    (t1804, t1808, t1811, t1813, t1818, t1822, t1825, t1828, t1829, t1832, t1837, t1843)
}
