//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2430/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2430<F: Float>(t90: F, t29: F, t11149: F, t78: F, t12267: F, t81: F, t46: F, t47: F, t58: F, t59: F, t2681: F, t64: F) -> (F, F, F, F, F, F) {
    let t45970 = t90 * t90;
    let t45972 = t29 / t45970;
    let t46001 = F::new(1.0) / t78 / t11149;
    let t46014 = F::new(1.0) / t81 / t12267;
    let t46063 = t46 * t46;
    let t46065 = F::new(1.0) / t47 / t46063;
    let t46072 = t58 * t58;
    let t46074 = F::new(1.0) / t59 / t46072;
    let t46089 = t64 * t2681;
    (t45972, t46001, t46014, t46065, t46074, t46089)
}
