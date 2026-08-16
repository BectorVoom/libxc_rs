//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta70 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk420;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk421;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk422;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk423;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk424;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk425;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk426;
use chunk7::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk427;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta70<F: Float>(t1347: F, t1799: F, t1819: F, t546: F, t548: F, t550: F, t1343: F, t820: F, t1367: F, t1315: F, t1327: F, t1341: F, t1360: F, t1363: F, t1811: F, t1815: F, t559: F, t539: F, t1380: F, t553: F, t1336: F, t1814: F, t544: F, t564: F, t1378: F, t1375: F, t1808: F, t568: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1821, t1824) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk420::<F>(t1347, t1799, t1819, t546, t548);
        let t1825 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk421::<F>(t1824, t550);
        let t1827 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk422::<F>(t1343, t1825, t820);
        let t1831 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk423::<F>(t1367, t1799, t820);
        let t1834 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk424::<F>(t1315, t1327, t1341, t1360, t1363, t1811, t1815, t1827, t1831, t559);
        let (t1835, t1838, t1840, t1842) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk425::<F>(t1834, t539, t1380, t1825, t553, t1336, t1814, t544, t564);
        let t1843 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk426::<F>(t1378, t1842);
        let t1845 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk427::<F>(t1375, t1808, t1835, t1843, t568);
    (t1821, t1824, t1825, t1827, t1831, t1834, t1835, t1838, t1840, t1842, t1843, t1845)
}
