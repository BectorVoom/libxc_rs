//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta278 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1558;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1559;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta278(t813: f64, t236: f64, t232: f64, t2632: f64, t2639: f64, t2686: f64, t2697: f64, t2703: f64, t842: f64, t9612: f64, t2617: f64, t2696: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9970, t9971, t9972, t9975) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1558(t813, t236, t232, t2632);
        let (t9986, t9988, t9990, t9993) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1559(t2639, t2686, t2697, t2703, t842, t9612, t2617, t2696);
    (t9970, t9971, t9972, t9975, t9986, t9988, t9990, t9993)
}
