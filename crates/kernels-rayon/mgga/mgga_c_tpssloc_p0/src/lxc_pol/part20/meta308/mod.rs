//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta308 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1556;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1557;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1558;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta308(t3314: f64, t422: f64, t11191: f64, t11275: f64, t1146: f64, t3399: f64, t3402: f64, t448: f64, t11129: f64, t1164: f64, t3411: f64, t3415: f64, t445: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11277, t11278, t11280, t11282) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1556(t3314, t422, t11191, t11275, t1146, t3399);
        let t11285 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1557(t3402, t448);
        let (t11286, t11288, t11290, t11292) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1558(t11129, t11282, t11285, t1164, t3411, t3415, t3399, t445);
    (t11277, t11278, t11280, t11282, t11285, t11286, t11288, t11290, t11292)
}
