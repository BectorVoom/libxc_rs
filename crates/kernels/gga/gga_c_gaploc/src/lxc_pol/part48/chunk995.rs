//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 995/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk995<F: Float>(t3566: F, t9333: F, t2365: F, t35913: F, t4391: F, t36274: F, t6963: F, t13437: F, t1562: F, t4614: F, t42412: F, t42431: F) -> (F, F, F, F, F, F) {
    let t46778 = F::new(0.25025342966295298669e1) * t3566 * t9333;
    let t46784 = t4391 * t2365 * t35913;
    let t46785 = F::new(0.59584149919750711116e-1) * t46784;
    let t46787 = t6963 * t2365 * t36274;
    let t46788 = F::new(0.29792074959875355558e-1) * t46787;
    let t46792 = F::new(0.92023022289409799224e1) * t1562 * t4614 * t13437;
    let t46793 = F::new(0.11916829983950142223e0) * t42412;
    let t46799 = F::new(0.19171462976960374838e0) * t42431;
    (t46778, t46785, t46788, t46792, t46793, t46799)
}
