//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 932/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk932<F: Float>(t11402: F, t2437: F, t13261: F, t1572: F, t4673: F, t11485: F, t3377: F, t3566: F, t9333: F, t2365: F, t35913: F, t4391: F) -> (F, F, F, F, F) {
    let t46767 = F::new(0.35750489951850426669e0) * t2437 * t11402;
    let t46773 = F::new(0.47667319935800568892e0) * t1572 * t4673 * t13261;
    let t46775 = F::new(0.25025342966295298669e1) * t11485 * t3377;
    let t46778 = F::new(0.25025342966295298669e1) * t3566 * t9333;
    let t46784 = t4391 * t2365 * t35913;
    (t46767, t46773, t46775, t46778, t46784)
}
