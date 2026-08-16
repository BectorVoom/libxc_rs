//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3817/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3817(t46989: f64, t46993: f64, t22483: f64, t39747: f64, t39750: f64, t39756: f64, t39760: f64, t4135: f64, t46988: f64, t46992: f64, t46996: f64, t46998: f64, t5541: f64) -> (f64, f64, f64) {
    let t73379 = 0.70178683471615754484e1_f64 * t46989;
    let t73380 = 0.10389515463408878255e3_f64 * t46993;
    let t73383 = -t22483 * t4135 * t5541 + t39747 + t39750 + t39756 + t39760 + t46988 + t46992 + t46996 - t46998 + t73379 - t73380;
    (t73379, t73380, t73383)
}
