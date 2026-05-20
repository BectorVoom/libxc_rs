//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3811/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3811<F: Float>(t46279: F, t46281: F, t46286: F, t46302: F, t3857: F, t6801: F, t14304: F, t21969: F, t39419: F, t39422: F, t4139: F, t4140: F, t46289: F, t46297: F, t5541: F, t5542: F) -> (F, F, F, F, F, F) {
    let t73314 = F::new(24.0) * t46279;
    let t73315 = F::new(120.0) * t46281;
    let t73316 = F::cast_from(0.11696447245269292414e1_f64) * t46286;
    let t73317 = F::new(24.0) * t46302;
    let t73321 = t3857 * t6801;
    let t73322 = F::new(20.0) * t73321;
    let t73326 = -F::new(2.0) * t14304 * t5541 * t5542 + F::new(6.0) * t21969 * t4139 * t4140 - t39419 - t39422 + t46289 - t46297 - t73314 + t73315 - t73316 - t73317 + t73322;
    (t73314, t73315, t73316, t73317, t73322, t73326)
}
