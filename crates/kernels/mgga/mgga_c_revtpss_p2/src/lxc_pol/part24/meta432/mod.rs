//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta432 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1382;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta432<F: Float>(t46072: F, t59: F, t2681: F, t64: F, t112: F, t10207: F, t111: F, t36227: F, t36415: F, t39454: F, t521: F, t1333: F, t9413: F) -> (F, F, F, F, F, F, F, F) {
        let (t46074, t46090, t46143, t46157, t46196, t46212, t46292, t46297) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1382::<F>(t46072, t59, t2681, t64, t112, t10207, t111, t36227, t36415, t39454, t521, t1333, t9413);
    (t46074, t46090, t46143, t46157, t46196, t46212, t46292, t46297)
}
