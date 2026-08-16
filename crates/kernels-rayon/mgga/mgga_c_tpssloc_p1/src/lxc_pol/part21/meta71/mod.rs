//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta71 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk519;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk520;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk521;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk522;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk523;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk524;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk525;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk526;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta71(t1426: f64, t33: f64, t1409: f64, t634: f64, t638: f64, t72: f64, t1411: f64, t66: f64, t80: f64, t5: f64, t1406: f64, t605: f64, t86: f64, t112: f64, t1408: f64, t95: f64, t50: f64, tau1: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1427 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk519(t1426, t33);
        let (t1430, t1431, t1434) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk520(t1409, t634, t638, t72);
        let t1437 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk521(t1411, t1427, t1434, t66, t80);
        let t1441 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk522(t5, t1406, t1437, t605, t86);
        let t1442 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk523(t112, t1441);
        let t1444 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk524(t1408);
        let (t1445, t1447) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk525(t1444, t95, t50, tau1);
        let t1449 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk526(t1444);
    (t1427, t1430, t1431, t1434, t1437, t1441, t1442, t1444, t1445, t1447, t1449)
}
