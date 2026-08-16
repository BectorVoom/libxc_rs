//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta76 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk532;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk533;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk534;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta76<F: Float>(t1545: F, t1559: F, t1561: F, t1569: F, t1574: F, t1581: F, t300: F, t311: F, t924: F, t943: F, t1580: F, t942: F, t951: F, t959: F, t1409: F, t978: F, t977: F, t1554: F, t906: F, t340: F, t343: F, t974: F, t971: F, t973: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t1585, t1587, t1589) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk532::<F>(t1545, t1559, t1561, t1569, t1574, t1581, t300, t311, t924, t943, t1580, t942, t951);
        let (t1591, t1592, t1593, t1597) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk533::<F>(t1589, t959, t1409, t978, t977, t1554, t906);
        let (t1599, t1600, t1603) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk534::<F>(t1597, t340, t343, t974, t1593, t971, t973);
    (t1585, t1587, t1589, t1591, t1592, t1593, t1597, t1599, t1600, t1603)
}
