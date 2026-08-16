//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2620/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2620(t46977: f64, t46979: f64, t46981: f64, t46983: f64, t46989: f64, t46993: f64, t187: f64, t48216: f64, t13597: f64, t2516: f64, t39747: f64, t39750: f64, t39756: f64, t39760: f64, t46988: f64, t46992: f64, t46996: f64, t46998: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t48247 = 360.0_f64 * t46977;
    let t48248 = 96.0_f64 * t46979;
    let t48249 = 4.0_f64 * t46981;
    let t48250 = 24.0_f64 * t46983;
    let t48251 = 0.10526802520742363173e2_f64 * t46989;
    let t48252 = 0.15584273195113317383e3_f64 * t46993;
    let t48254 = 0.19751673498613801407e-1_f64 * t48216 * t187;
    let t48255 = t13597 * t2516;
    let t48256 = 0.17544670867903938621e1_f64 * t48255;
    let t48257 = -t48247 - t48248 - t48249 - t48250 + t39747 + t46988 + t48251 + t46992 + t39750 + t39756 + t39760 - t48252 + t46996 - t46998 + t48254 - t48256;
    (t48247, t48248, t48249, t48250, t48251, t48252, t48254, t48256, t48257)
}
