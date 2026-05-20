//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta456 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1740;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1741;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta456<F: Float>(t1340: F, t40086: F, t4038: F, t9318: F, t1337: F, t40101: F, t9323: F, t40097: F, t39816: F, t1333: F, t9855: F, t19: F, t2237: F, t521: F, t39747: F, t39750: F, t39756: F, t39760: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t46988, t46990, t46992, t46994, t46996, t46998, t47000, t47003) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1740::<F>(t1340, t40086, t4038, t9318, t1337, t40101, t9323, t40097, t39816, t1333, t9855, t19, t2237, t521);
        let t47004 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1741::<F>(t39747, t39750, t39756, t39760, t46988, t46990, t46992, t46994, t46996, t46998, t47000, t47003);
    (t46988, t46990, t46992, t46994, t46996, t46998, t47000, t47003, t47004)
}
