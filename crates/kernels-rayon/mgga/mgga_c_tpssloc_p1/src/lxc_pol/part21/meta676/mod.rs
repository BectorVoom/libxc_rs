//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta676 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2481;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2482;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta676(t39: f64, t9287: f64, t51: f64, t9300: f64, t12566: f64, t604: f64, t2239: f64, t3951: f64, t4199: f64, t9919: f64, t12887: f64, t67: f64, t758: f64, t9892: f64, t13123: f64, t9882: f64, t9888: f64, t118: f64, t2375: f64, t4095: f64, t9905: f64, t2517: f64, t3966: f64, t707: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t45970, t45974, t46099, t46104, t46125, t46128) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2481(t39, t9287, t51, t9300, t12566, t604, t2239, t3951, t4199, t9919, t12887, t67, t758);
        let (t46130, t46132, t46134, t46137, t46196, t46206) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2482(t4199, t9892, t13123, t9882, t9888, t118, t2375, t4095, t9905, t2517, t3966, t707);
    (t45970, t45974, t46099, t46104, t46125, t46128, t46130, t46132, t46134, t46137, t46196, t46206)
}
