//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta197 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1224;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1225;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1226;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta197(t3297: f64, t4724: f64, t136: f64, t1113: f64, t4729: f64, t4733: f64, t3238: f64, t3282: f64, t3294: f64, t3295: f64, t4721: f64, t4726: f64, t4731: f64, t4735: f64, t4749: f64, t4757: f64, t4765: f64, t4767: f64, t4770: f64, t1118: f64, t1099: f64, t1670: f64, t3315: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4772, t4773, t4775, t4776, t4778, t4779, t4781) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1224(t3297, t4724, t136, t1113, t4729, t4733, t3238, t3282, t3294, t3295, t4721, t4726, t4731, t4735, t4749, t4757, t4765, t4767, t4770);
        let t4782 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1225(t1118, t4781);
        let (t4784, t4785) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1226(t1099, t4782, t1670, t3315);
    (t4772, t4773, t4775, t4776, t4778, t4779, t4781, t4782, t4784, t4785)
}
