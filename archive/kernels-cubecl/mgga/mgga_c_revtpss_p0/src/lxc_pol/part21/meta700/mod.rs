//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta700 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2522;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2523;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta700<F: Float>(t10285: F, t596: F, t10293: F, t592: F, t25: F, t40649: F, t10296: F, t602: F, t2240: F, t2246: F, t10308: F, t599: F, t90: F, t29: F, t11149: F, t78: F, t12267: F, t81: F, t46: F, t47: F, t58: F, t59: F, t2681: F, t64: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t45945, t45949, t45952, t45955, t45958, t45963) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2522::<F>(t10285, t596, t10293, t592, t25, t40649, t10296, t602, t2240, t2246, t10308, t599);
        let (t45972, t46001, t46014, t46065, t46074, t46089) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2523::<F>(t90, t29, t11149, t78, t12267, t81, t46, t47, t58, t59, t2681, t64);
    (t45945, t45949, t45952, t45955, t45958, t45963, t45972, t46001, t46014, t46065, t46074, t46089)
}
