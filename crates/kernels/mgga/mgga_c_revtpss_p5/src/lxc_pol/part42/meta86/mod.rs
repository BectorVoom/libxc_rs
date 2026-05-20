//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta86 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk502;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk503;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk504;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk505;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk506;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk507;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta86<F: Float>(t1803: F, t467: F, t1264: F, t1715: F, t247: F, t1221: F, t1222: F, t1235: F, t1247: F, t1258: F, t1261: F, t1778: F, t1782: F, t1786: F, t1791: F, t1797: F, t464: F, t484: F, t225: F, t494: F, t1280: F, t1774: F, t1287: F, t1794: F, t487: F, t489: F, t1234: F, t1285: F, t1770: F, t460: F, t490: F, t1277: F, t265: F, t502: F, t1210: F, t1274: F, t1775: F, t495: F, t1300: F, t1587: F, t1721: F, t1735: F, t1761: F, t1763: F, t1767: F, t198: F, t336: F, t33: F, t1469: F, t1711: F, t504: F, t57: F, t1709: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1804, t1808) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk502::<F>(t1803, t467, t1264, t1715, t247);
        let t1811 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk503::<F>(t1221, t1222, t1235, t1247, t1258, t1261, t1778, t1782, t1786, t1791, t1797, t1804, t1808, t464, t484);
        let (t1813, t1818, t1822, t1825, t1828) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk504::<F>(t1811, t225, t494, t1280, t1774, t1287, t1794, t487, t489, t1234, t1285, t1770, t460, t490);
        let t1829 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk505::<F>(t1277, t1828);
        let (t1832, t1837) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk506::<F>(t265, t502, t1210, t1274, t1770, t1775, t1813, t1829, t460, t495, t1300, t1587, t1721, t1735, t1761, t1763, t1767, t198, t336);
        let t1843 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk507::<F>(t33, t1469, t1587, t1711, t1837, t265, t504, t57, t1709, dens_threshold, rho1, zeta_threshold);
    (t1804, t1808, t1811, t1813, t1818, t1822, t1825, t1828, t1829, t1832, t1837, t1843)
}
