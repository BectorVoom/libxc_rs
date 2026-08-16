//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta88 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk575;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk576;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk577;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk578;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk579;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk580;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk581;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta88<F: Float>(t1834: F, t539: F, t1380: F, t1825: F, t553: F, t1336: F, t1814: F, t544: F, t564: F, t1378: F, t1375: F, t1808: F, t568: F, t1274: F, t1276: F, t1288: F, t1293: F, t1296: F, t1297: F, t1390: F, t1789: F, t1791: F, t1799: F, t193: F, t533: F, t680: F, t705: F, t113: F, t1442: F, t1459: F, t1774: F, t1778: F, t510: F, t513: F, t574: F, t652: F, t3: F, t1401: F, t1458: F, t577: F, t33: F, t605: F, t71: F, t79: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1835, t1838, t1840, t1842) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk575::<F>(t1834, t539, t1380, t1825, t553, t1336, t1814, t544, t564);
        let t1843 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk576::<F>(t1378, t1842);
        let t1845 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk577::<F>(t1375, t1808, t1835, t1843, t568);
        let t1849 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk578::<F>(t1274, t1276, t1288, t1293, t1296, t1297, t1390, t1789, t1791, t1799, t1845, t193, t533, t680, t705);
        let t1851 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk579::<F>(t113, t1442, t1459, t1774, t1778, t1849, t510, t513, t574, t652);
        let (t1852, t1858, t1860) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk580::<F>(t1851, t3, t1401, t1458, t577, t33, t605);
        let t1864 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk581::<F>(t71, t79);
    (t1835, t1838, t1840, t1842, t1843, t1845, t1849, t1851, t1852, t1858, t1860, t1864)
}
