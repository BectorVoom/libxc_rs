//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta86 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk548;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk549;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk550;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk551;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk552;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk553;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk554;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk555;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta86<F: Float>(t1811: F, t225: F, t494: F, t1280: F, t1774: F, t1287: F, t1794: F, t487: F, t489: F, t1234: F, t1285: F, t1770: F, t460: F, t490: F, t1277: F, t1210: F, t1274: F, t1775: F, t495: F, t33: F, t265: F, t502: F, t1300: F, t1587: F, t1721: F, t1735: F, t1761: F, t1763: F, t1767: F, t198: F, t336: F, t1469: F, t1711: F, t504: F, t57: F, dens_threshold: F, rho1: F, zeta_threshold: F, t1709: F, t1312: F, t1502: F, t1518: F, t30: F, t1468: F, t513: F, t516: F, t162: F, t189: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1813, t1818, t1822, t1825, t1828) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk548::<F>(t1811, t225, t494, t1280, t1774, t1287, t1794, t487, t489, t1234, t1285, t1770, t460, t490);
        let t1829 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk549::<F>(t1277, t1828);
        let t1832 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk550::<F>(t1210, t1274, t1770, t1775, t1813, t1829, t460, t495);
        let (t1837, t1842) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk551::<F>(t33, t265, t502, t1300, t1587, t1721, t1735, t1761, t1763, t1767, t1832, t198, t336, t1469, t1711, t504, t57, dens_threshold, rho1, zeta_threshold);
        let t1843 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk552::<F>(t1709, t1842);
        let t1847 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk553::<F>(t1312, t1502, t1518);
        let t1856 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk554::<F>(t30, t33, t1468, t513, t1711, t516, t162, zeta_threshold);
        let t1857 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk555::<F>(t1856, t189);
    (t1813, t1818, t1822, t1825, t1828, t1829, t1832, t1837, t1843, t1847, t1856, t1857)
}
