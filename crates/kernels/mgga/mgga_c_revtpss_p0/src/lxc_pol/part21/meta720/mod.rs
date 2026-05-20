//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta720 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2559;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta720<F: Float>(t4038: F, t9318: F, t1337: F, t40101: F, t9323: F, t1340: F, t40097: F, t39816: F, t19: F, t2237: F, t521: F, t1331: F, t9342: F) -> (F, F, F, F, F, F, F) {
        let (t46989, t46992, t46993, t46996, t46998, t47003, t47005) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2559::<F>(t4038, t9318, t1337, t40101, t9323, t1340, t40097, t39816, t19, t2237, t521, t1331, t9342);
    (t46989, t46992, t46993, t46996, t46998, t47003, t47005)
}
