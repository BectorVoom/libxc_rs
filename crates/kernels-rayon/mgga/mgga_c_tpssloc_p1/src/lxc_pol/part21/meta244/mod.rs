//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta244 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1433;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1434;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1435;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1436;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1437;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1438;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1439;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta244(t1099: f64, t6021: f64, t3315: f64, t5988: f64, t3313: f64, t3319: f64, t4721: f64, t5973: f64, t5977: f64, t5981: f64, t1682: f64, t1137: f64, t3339: f64, t3346: f64, t4770: f64, t5993: f64, t6000: f64, t6006: f64, t6008: f64, t6012: f64, t6015: f64, t6018: f64, t3359: f64, t3363: f64, t449: f64, t1694: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6023, t6024) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1433(t1099, t6021, t3315, t5988);
        let (t6026, t6031, t6036, t6037) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1434(t3313, t6024, t3319, t4721, t5973, t5977, t5981, t1682, t1137);
        let t6052 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1435(t3339, t3346, t4721, t4770, t5973, t5977, t5981, t5993, t6000, t6006, t6008, t6012, t6015, t6018);
        let t6053 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1436(t1137, t6052);
        let t6056 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1437(t3359, t6036);
        let t6063 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1438(t3363, t4721, t5973, t5977, t5981);
        let (t6064, t6068) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1439(t449, t6063, t1694);
    (t6023, t6024, t6026, t6031, t6036, t6037, t6052, t6053, t6056, t6063, t6064, t6068)
}
