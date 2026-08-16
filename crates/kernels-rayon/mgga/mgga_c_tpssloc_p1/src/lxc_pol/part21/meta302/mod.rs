//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta302 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1641;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1642;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1643;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1644;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1645;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta302(t407: f64, t11135: f64, t410: f64, t417: f64, t1097: f64, t3311: f64, t409: f64, t3314: f64, t422: f64, t1146: f64, t3399: f64, t3402: f64, t448: f64, t445: f64, t1143: f64, t3375: f64, t1124: f64, t3331: f64, t440: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11243, t11247, t11265, t11274, t11275) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1641(t407, t11135, t410, t417, t1097, t3311, t409);
        let (t11277, t11282) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1642(t3314, t422, t1146, t3399);
        let t11285 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1643(t3402, t448);
        let t11292 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1644(t3399, t445);
        let (t11297, t11303, t11310) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1645(t1143, t3375, t1124, t3331, t11282, t440);
    (t11243, t11247, t11265, t11274, t11275, t11277, t11282, t11285, t11292, t11297, t11303, t11310)
}
