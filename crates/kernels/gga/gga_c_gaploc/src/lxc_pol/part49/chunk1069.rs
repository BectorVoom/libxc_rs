//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1069/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1069<F: Float>(t46845: F, t605: F, t1377: F, t13836: F, t41572: F, t3689: F, t874: F) -> (F, F, F, F) {
    let t46846 = t46845 * t605;
    let t46847 = t1377 * t13836;
    let t46848 = F::new(2.0) * t41572;
    let t46849 = t3689 * t874;
    (t46846, t46847, t46848, t46849)
}
