//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta162 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk781;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk782;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta162(t1268: f64, t2314: f64, t2363: f64, t5113: f64, t671: f64, t9347: f64, t9348: f64, t9351: f64, t9416: f64, t195: f64, t40: f64, t2433: f64, t607: f64, t52: f64, t2250: f64, t73: f64, t9258: f64, t9288: f64, t197: f64, t2440: f64, t76: f64, t145: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t9419, t9427, t9430) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk781(t1268, t2314, t2363, t5113, t671, t9347, t9348, t9351, t9416, t195, t40, t2433, t607);
        let (t9438, t9441, t9448, t9449) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk782(t40, t52, t2250, t73, t9258, t9288, t9427, t9430, t197, t2440, t607, t76, t145, zeta_threshold);
    (t9419, t9427, t9430, t9438, t9441, t9448, t9449)
}
