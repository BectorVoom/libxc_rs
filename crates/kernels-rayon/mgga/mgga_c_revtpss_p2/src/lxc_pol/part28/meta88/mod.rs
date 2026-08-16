//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta88 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk551;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk552;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk553;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk554;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk555;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk556;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta88(t1811: f64, t225: f64, t494: f64, t1280: f64, t1774: f64, t1287: f64, t1794: f64, t487: f64, t489: f64, t1234: f64, t1285: f64, t1770: f64, t460: f64, t490: f64, t1277: f64, t265: f64, t502: f64, t1210: f64, t1274: f64, t1775: f64, t495: f64, t1300: f64, t1587: f64, t1721: f64, t1735: f64, t1761: f64, t1763: f64, t1767: f64, t198: f64, t336: f64, t33: f64, t1469: f64, t1711: f64, t504: f64, t57: f64, t1709: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t30: f64, t1312: f64, t1502: f64, t1518: f64, t1468: f64, t513: f64, t516: f64, t162: f64, t189: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1813, t1818, t1822, t1825, t1828) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk551(t1811, t225, t494, t1280, t1774, t1287, t1794, t487, t489, t1234, t1285, t1770, t460, t490);
        let t1829 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk552(t1277, t1828);
        let (t1832, t1837) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk553(t265, t502, t1210, t1274, t1770, t1775, t1813, t1829, t460, t495, t1300, t1587, t1721, t1735, t1761, t1763, t1767, t198, t336);
        let t1843 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk554(t33, t1469, t1587, t1711, t1837, t265, t504, t57, t1709, dens_threshold, rho1, zeta_threshold);
        let (t1847, t1856) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk555(t30, t33, t1312, t1502, t1518, t1468, t513, t1711, t516, t162, zeta_threshold);
        let t1857 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk556(t1856, t189);
    (t1813, t1818, t1822, t1825, t1828, t1829, t1832, t1837, t1843, t1847, t1856, t1857)
}
