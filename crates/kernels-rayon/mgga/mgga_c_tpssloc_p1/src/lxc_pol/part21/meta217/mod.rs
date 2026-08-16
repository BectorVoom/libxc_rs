//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta217 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1319;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1320;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1321;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1322;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1323;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1324;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1325;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1326;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta217(t1365: f64, t1799: f64, t1307: f64, t1347: f64, t5187: f64, t1345: f64, t1348: f64, t1819: f64, t1821: f64, t5272: f64, t5278: f64, t546: f64, t548: f64, t550: f64, t1343: f64, t820: f64, t1352: f64, t5248: f64, t5249: f64, t120: f64, t3805: f64, t1831: f64, t3866: f64, t3870: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5279, t5280, t5283, t5286) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1319(t1365, t1799, t1307, t1347, t5187, t1345, t1348, t1819, t1821, t5272, t5278, t546, t548);
        let t5287 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1320(t5286, t550);
        let t5289 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1321(t1343, t5287, t820);
        let t5293 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1322(t1352, t5248, t5249);
        let t5301 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1323(t120, t1799);
        let t5303 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1324(t1352, t3805, t5301);
        let (t5306, t5308) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1325(t1831, t3866, t1307, t1799);
        let t5310 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1326(t3870, t5308, t820);
    (t5279, t5280, t5283, t5286, t5287, t5289, t5293, t5301, t5303, t5306, t5308, t5310)
}
