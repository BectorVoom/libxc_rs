//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta87 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk626;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk627;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk628;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk629;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk630;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk631;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk632;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk633;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta87(t265: f64, t502: f64, t1210: f64, t1274: f64, t1770: f64, t1775: f64, t1813: f64, t1829: f64, t460: f64, t495: f64, t1300: f64, t1587: f64, t1721: f64, t1735: f64, t1761: f64, t1763: f64, t1767: f64, t198: f64, t336: f64, t33: f64, t1469: f64, t1711: f64, t504: f64, t57: f64, t1709: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t1312: f64, t1502: f64, t1518: f64, t30: f64, t1468: f64, t513: f64, t516: f64, t162: f64, t189: f64, t512: f64, t187: f64, t1344: f64, t1348: f64, t124: f64, t800: f64, t1319: f64, t1322: f64, t1334: f64, t1339: f64, t1342: f64, t225: f64, t679: f64, t704: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1832, t1837) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk626(t265, t502, t1210, t1274, t1770, t1775, t1813, t1829, t460, t495, t1300, t1587, t1721, t1735, t1761, t1763, t1767, t198, t336);
        let t1843 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk627(t33, t1469, t1587, t1711, t1837, t265, t504, t57, t1709, dens_threshold, rho1, zeta_threshold);
        let t1847 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk628(t1312, t1502, t1518);
        let t1856 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk629(t30, t33, t1468, t513, t1711, t516, t162, zeta_threshold);
        let t1857 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk630(t1856, t189);
        let (t1858, t1860, t1868) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk631(t30, t33, t1857, t512, t1856, t187, t1344, t1468, t1348, t1711, zeta_threshold);
        let t1872 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk632(t124, t1868);
        let (t1873, t1877) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk633(t1872, t800, t1319, t1322, t1334, t1339, t1342, t1858, t1860, t225, t679, t704);
    (t1832, t1837, t1843, t1847, t1856, t1857, t1858, t1860, t1868, t1872, t1873, t1877)
}
