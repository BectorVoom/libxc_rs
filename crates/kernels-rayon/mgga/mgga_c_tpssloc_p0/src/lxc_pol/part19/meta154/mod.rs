//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta154 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk765;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk766;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta154(t9212: f64, t591: f64, t9: f64, t21: f64, t587: f64, t14: f64, t598: f64, t2230: f64, t594: f64, t2229: f64, t3: f64, t19: f64, t9211: f64, t2233: f64, t604: f64, t2239: f64, t601: f64, t83: f64, t84: f64, t85: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9213, t9214, t9215, t9216, t9217, t9218, t9219, t9221, t9223, t9225) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk765(t9212, t591, t9, t21, t587, t14, t598, t2230, t594, t2229, t3, t19);
        let (t9226, t9228, t9231, t9238) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk766(t9211, t9213, t9215, t9217, t9219, t9221, t9225, t2233, t604, t2239, t601, t83, t84, t85);
    (t9214, t9216, t9218, t9223, t9226, t9228, t9231, t9238)
}
