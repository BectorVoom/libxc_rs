//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta85 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk597;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk598;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk599;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk600;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk601;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk602;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk603;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta85<F: Float>(t1810: F, t210: F, t1807: F, t225: F, t554: F, t1274: F, t1276: F, t1288: F, t1293: F, t1296: F, t1789: F, t1791: F, t680: F, t705: F, t1347: F, t1799: F, t546: F, t548: F, t550: F, t1343: F, t820: F, t1367: F, t1315: F, t1327: F, t1341: F, t1360: F, t1363: F, t559: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t1811, t1814) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk597::<F>(t1810, t210, t1807, t225);
        let (t1815, t1819) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk598::<F>(t1814, t554, t1274, t1276, t1288, t1293, t1296, t1789, t1791, t225, t680, t705);
        let (t1821, t1824) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk599::<F>(t1347, t1799, t1819, t546, t548);
        let t1825 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk600::<F>(t1824, t550);
        let t1827 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk601::<F>(t1343, t1825, t820);
        let t1831 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk602::<F>(t1367, t1799, t820);
        let t1834 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk603::<F>(t1315, t1327, t1341, t1360, t1363, t1811, t1815, t1827, t1831, t559);
    (t1811, t1814, t1815, t1819, t1821, t1824, t1825, t1827, t1831, t1834)
}
