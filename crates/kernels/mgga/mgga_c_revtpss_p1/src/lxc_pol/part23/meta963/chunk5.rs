//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3260/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3260<F: Float>(t22860: F, t47194: F, t1410: F, t46760: F, t46787: F, t46800: F, t46810: F, t46817: F, t46820: F, t46824: F, t48638: F, t48645: F, t48669: F, t48686: F, t48691: F, t48692: F, t48696: F, t48700: F, t5591: F, t6836: F, t74364: F, t828: F, t9942: F) -> F {
    let t85865 = t47194 * t22860;
    let t85871 = -F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t74364 + t48638 + F::cast_from(0.68026775414003982664e-1_f64) * t48645 - F::cast_from(0.77173232612525526549e-1_f64) * t1410 * t9942 * t828 * t6836 * t5591 + F::cast_from(0.60023625365297631763e-2_f64) * t85865 - t46760 + t48669 - F::cast_from(0.80328230880474379776e-6_f64) * t46787 + t48686 - t48691 - F::cast_from(0.91464571985215438873e-3_f64) * t48692 + F::cast_from(0.5421477899694558815e-4_f64) * t48696 + F::cast_from(0.54214778996945588148e-4_f64) * t48700 + t46800 + t46810 - t46817 + t46820 - t46824;
    t85871
}
