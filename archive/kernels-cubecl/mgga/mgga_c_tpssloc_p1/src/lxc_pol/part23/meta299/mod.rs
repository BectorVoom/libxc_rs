//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta299 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1026;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1027;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1028;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta299<F: Float>(t21429: F, t21479: F, t225: F, t68: F, t369: F, t14211: F, t17712: F, t4582: F, t21126: F, t977: F, t21122: F, t2979: F, t10377: F, t10385: F, t10480: F, t10876: F, t10883: F, t14508: F, t14511: F, t17612: F, t17616: F, t21393: F, t21398: F, t21405: F, t3130: F, t378: F, t5875: F, t5880: F, t973: F, t1616: F, t1409: F, t5398: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t21480, t21481, t21482, t21483, t21486, t21487, t21490, t21493) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1026::<F>(t21429, t21479, t225, t68, t369, t14211, t17712, t4582, t21126, t977, t21122, t2979);
        let t21498 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1027::<F>(t10377, t10385, t10480, t10876, t10883, t14508, t14511, t17612, t17616, t21393, t21398, t21405, t21483, t21487, t21490, t21493, t3130, t378, t5875, t5880, t973);
        let (t21502, t21503, t21510) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1028::<F>(t1616, t17712, t4582, t1409, t5398);
    (t21480, t21481, t21482, t21483, t21486, t21487, t21498, t21502, t21503, t21510)
}
