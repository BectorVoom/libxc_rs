//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3260/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3260(t22860: f64, t47194: f64, t1410: f64, t46760: f64, t46787: f64, t46800: f64, t46810: f64, t46817: f64, t46820: f64, t46824: f64, t48638: f64, t48645: f64, t48669: f64, t48686: f64, t48691: f64, t48692: f64, t48696: f64, t48700: f64, t5591: f64, t6836: f64, t74364: f64, t828: f64, t9942: f64) -> f64 {
    let t85865 = t47194 * t22860;
    let t85871 = -7.0_f64 / 16.0_f64 * t74364 + t48638 + 0.68026775414003982664e-1_f64 * t48645 - 0.77173232612525526549e-1_f64 * t1410 * t9942 * t828 * t6836 * t5591 + 0.60023625365297631763e-2_f64 * t85865 - t46760 + t48669 - 0.80328230880474379776e-6_f64 * t46787 + t48686 - t48691 - 0.91464571985215438873e-3_f64 * t48692 + 0.5421477899694558815e-4_f64 * t48696 + 0.54214778996945588148e-4_f64 * t48700 + t46800 + t46810 - t46817 + t46820 - t46824;
    t85871
}
