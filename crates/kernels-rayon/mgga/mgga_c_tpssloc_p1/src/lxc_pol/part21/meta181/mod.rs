//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta181 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1152;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1153;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta181(t4347: f64, t882: f64, t123: f64, t2765: f64, t2766: f64, t4335: f64, t4340: f64, t4345: f64, t291: f64, t1543: f64, t892: f64, t914: f64, t1557: f64, t2787: f64, t912: f64, t2792: f64, t1547: f64, t2798: f64, t896: f64, t2802: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4348, t4349, t4351, t4353, t4354) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1152(t4347, t882, t123, t2765, t2766, t4335, t4340, t4345, t291, t1543, t892);
        let (t4356, t4358, t4359, t4361, t4362, t4363, t4370) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1153(t4354, t914, t1557, t2787, t912, t2792, t1547, t2798, t896, t2766, t2802, t4335, t4340, t4345, t4349);
    (t4348, t4349, t4351, t4353, t4354, t4356, t4358, t4359, t4361, t4362, t4363, t4370)
}
