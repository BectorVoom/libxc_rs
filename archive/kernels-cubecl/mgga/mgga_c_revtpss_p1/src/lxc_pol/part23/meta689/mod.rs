//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta689 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2431;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta689<F: Float>(t46089: F, t112: F, t10199: F, t666: F, t10207: F, t111: F, t36227: F, t36415: F, t3860: F, t4029: F, t3857: F, t4038: F, t9387: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t46090, t46143, t46144, t46157, t46196, t46212, t46279, t46281, t46286) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2431::<F>(t46089, t112, t10199, t666, t10207, t111, t36227, t36415, t3860, t4029, t3857, t4038, t9387);
    (t46090, t46143, t46144, t46157, t46196, t46212, t46279, t46281, t46286)
}
