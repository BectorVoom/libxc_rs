//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta585 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2096;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta585(t16: f64, t39031: f64, t39: f64, t9287: f64, t51: f64, t9300: f64, t39033: f64, t39035: f64, t39037: f64, t39039: f64, t2239: f64, t3951: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t45869, t45870, t45970, t45974, t46085, t46086, t46087, t46088, t46089, t46090, t46104) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2096(t16, t39031, t39, t9287, t51, t9300, t39033, t39035, t39037, t39039, t2239, t3951);
    (t45869, t45870, t45970, t45974, t46085, t46086, t46087, t46088, t46089, t46090, t46104)
}
