//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta676 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2481;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2482;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta676<F: Float>(t39: F, t9287: F, t51: F, t9300: F, t12566: F, t604: F, t2239: F, t3951: F, t4199: F, t9919: F, t12887: F, t67: F, t758: F, t9892: F, t13123: F, t9882: F, t9888: F, t118: F, t2375: F, t4095: F, t9905: F, t2517: F, t3966: F, t707: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t45970, t45974, t46099, t46104, t46125, t46128) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2481::<F>(t39, t9287, t51, t9300, t12566, t604, t2239, t3951, t4199, t9919, t12887, t67, t758);
        let (t46130, t46132, t46134, t46137, t46196, t46206) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2482::<F>(t4199, t9892, t13123, t9882, t9888, t118, t2375, t4095, t9905, t2517, t3966, t707);
    (t45970, t45974, t46099, t46104, t46125, t46128, t46130, t46132, t46134, t46137, t46196, t46206)
}
