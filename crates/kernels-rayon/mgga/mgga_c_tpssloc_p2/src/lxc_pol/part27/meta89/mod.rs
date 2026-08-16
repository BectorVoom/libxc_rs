//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta89 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk576;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk577;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk578;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk579;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk580;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk581;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta89(t1834: f64, t539: f64, t1380: f64, t1825: f64, t553: f64, t1336: f64, t1814: f64, t544: f64, t564: f64, t1378: f64, t1375: f64, t1808: f64, t568: f64, t1274: f64, t1276: f64, t1288: f64, t1293: f64, t1296: f64, t1297: f64, t1390: f64, t1789: f64, t1791: f64, t1799: f64, t193: f64, t533: f64, t680: f64, t705: f64, t113: f64, t1442: f64, t1459: f64, t1774: f64, t1778: f64, t510: f64, t513: f64, t574: f64, t652: f64, t3: f64, t1401: f64, t1458: f64, t577: f64, t33: f64, t605: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1835, t1838, t1840, t1842) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk576(t1834, t539, t1380, t1825, t553, t1336, t1814, t544, t564);
        let t1843 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk577(t1378, t1842);
        let t1845 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk578(t1375, t1808, t1835, t1843, t568);
        let t1849 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk579(t1274, t1276, t1288, t1293, t1296, t1297, t1390, t1789, t1791, t1799, t1845, t193, t533, t680, t705);
        let t1851 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk580(t113, t1442, t1459, t1774, t1778, t1849, t510, t513, t574, t652);
        let (t1852, t1858, t1860) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk581(t1851, t3, t1401, t1458, t577, t33, t605);
    (t1835, t1838, t1840, t1842, t1843, t1845, t1849, t1851, t1852, t1858, t1860)
}
