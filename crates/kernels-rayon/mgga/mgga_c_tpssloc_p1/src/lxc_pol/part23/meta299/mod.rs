//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta299 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1026;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1027;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1028;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta299(t21429: f64, t21479: f64, t225: f64, t68: f64, t369: f64, t14211: f64, t17712: f64, t4582: f64, t21126: f64, t977: f64, t21122: f64, t2979: f64, t10377: f64, t10385: f64, t10480: f64, t10876: f64, t10883: f64, t14508: f64, t14511: f64, t17612: f64, t17616: f64, t21393: f64, t21398: f64, t21405: f64, t3130: f64, t378: f64, t5875: f64, t5880: f64, t973: f64, t1616: f64, t1409: f64, t5398: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21480, t21481, t21482, t21483, t21486, t21487, t21490, t21493) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1026(t21429, t21479, t225, t68, t369, t14211, t17712, t4582, t21126, t977, t21122, t2979);
        let t21498 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1027(t10377, t10385, t10480, t10876, t10883, t14508, t14511, t17612, t17616, t21393, t21398, t21405, t21483, t21487, t21490, t21493, t3130, t378, t5875, t5880, t973);
        let (t21502, t21503, t21510) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1028(t1616, t17712, t4582, t1409, t5398);
    (t21480, t21481, t21482, t21483, t21486, t21487, t21498, t21502, t21503, t21510)
}
