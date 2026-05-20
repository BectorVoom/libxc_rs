//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3817/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3817<F: Float>(t46989: F, t46993: F, t22483: F, t39747: F, t39750: F, t39756: F, t39760: F, t4135: F, t46988: F, t46992: F, t46996: F, t46998: F, t5541: F) -> (F, F, F) {
    let t73379 = F::cast_from(0.70178683471615754484e1_f64) * t46989;
    let t73380 = F::cast_from(0.10389515463408878255e3_f64) * t46993;
    let t73383 = -t22483 * t4135 * t5541 + t39747 + t39750 + t39756 + t39760 + t46988 + t46992 + t46996 - t46998 + t73379 - t73380;
    (t73379, t73380, t73383)
}
