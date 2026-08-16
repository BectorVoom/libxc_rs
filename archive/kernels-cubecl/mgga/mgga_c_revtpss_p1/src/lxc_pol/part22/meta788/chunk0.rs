//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2878/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2878<F: Float>(t58: F, t59: F, t2681: F, t64: F, t112: F, t10199: F, t666: F, t2289: F, t2341: F, t2367: F, t10207: F, t111: F) -> (F, F, F, F, F, F, F) {
    let t46072 = t58 * t58;
    let t46074 = F::cast_from(1.0_f64) / t59 / t46072;
    let t46089 = t64 * t2681;
    let t46090 = F::cast_from(20944.0_f64) / F::cast_from(81.0_f64) * t46089;
    let t46143 = F::cast_from(2618.0_f64) / F::cast_from(81.0_f64) * t46089 * t112;
    let t46144 = t10199 * t666;
    let t46146 = t2289 * t2341;
    let t46148 = t2289 * t2367;
    let t46157 = F::cast_from(1.0_f64) / t10207 / t111;
    (t46074, t46090, t46143, t46144, t46146, t46148, t46157)
}
