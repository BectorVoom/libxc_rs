//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta788 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2878;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta788<F: Float>(t58: F, t59: F, t2681: F, t64: F, t112: F, t10199: F, t666: F, t2289: F, t2341: F, t2367: F, t10207: F, t111: F) -> (F, F, F, F, F, F, F) {
        let (t46074, t46090, t46143, t46144, t46146, t46148, t46157) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2878::<F>(t58, t59, t2681, t64, t112, t10199, t666, t2289, t2341, t2367, t10207, t111);
    (t46074, t46090, t46143, t46144, t46146, t46148, t46157)
}
