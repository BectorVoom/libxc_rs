//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta431 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1381;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta431<F: Float>(t45949: F, t25: F, t40649: F, t90: F, t29: F, t11149: F, t78: F, t12267: F, t81: F, t46: F, t47: F, t58: F) -> (F, F, F, F, F, F, F) {
        let (t45950, t45952, t45972, t46001, t46014, t46065, t46072) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1381::<F>(t45949, t25, t40649, t90, t29, t11149, t78, t12267, t81, t46, t47, t58);
    (t45950, t45952, t45972, t46001, t46014, t46065, t46072)
}
