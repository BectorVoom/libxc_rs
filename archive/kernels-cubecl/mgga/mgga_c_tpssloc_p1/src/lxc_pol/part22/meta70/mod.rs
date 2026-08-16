//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta70 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk491;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk492;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk493;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk494;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk495;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk496;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk497;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk498;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta70<F: Float>(t1426: F, t33: F, t1409: F, t634: F, t638: F, t72: F, t1411: F, t66: F, t80: F, t5: F, t1406: F, t605: F, t86: F, t112: F, t1408: F, t95: F, t50: F, tau1: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1427 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk491::<F>(t1426, t33);
        let (t1430, t1431, t1433, t1434) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk492::<F>(t1409, t634, t638, t72);
        let t1437 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk493::<F>(t1411, t1427, t1434, t66, t80);
        let t1441 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk494::<F>(t5, t1406, t1437, t605, t86);
        let t1442 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk495::<F>(t112, t1441);
        let t1444 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk496::<F>(t1408);
        let (t1445, t1447) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk497::<F>(t1444, t95, t50, tau1);
        let t1449 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk498::<F>(t1444);
    (t1427, t1430, t1431, t1433, t1434, t1437, t1441, t1442, t1444, t1445, t1447, t1449)
}
