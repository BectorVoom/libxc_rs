//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2620/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2620<F: Float>(t46977: F, t46979: F, t46981: F, t46983: F, t46989: F, t46993: F, t187: F, t48216: F, t13597: F, t2516: F, t39747: F, t39750: F, t39756: F, t39760: F, t46988: F, t46992: F, t46996: F, t46998: F) -> (F, F, F, F, F, F, F, F, F) {
    let t48247 = F::new(360.0) * t46977;
    let t48248 = F::new(96.0) * t46979;
    let t48249 = F::new(4.0) * t46981;
    let t48250 = F::new(24.0) * t46983;
    let t48251 = F::cast_from(0.10526802520742363173e2_f64) * t46989;
    let t48252 = F::cast_from(0.15584273195113317383e3_f64) * t46993;
    let t48254 = F::cast_from(0.19751673498613801407e-1_f64) * t48216 * t187;
    let t48255 = t13597 * t2516;
    let t48256 = F::cast_from(0.17544670867903938621e1_f64) * t48255;
    let t48257 = -t48247 - t48248 - t48249 - t48250 + t39747 + t46988 + t48251 + t46992 + t39750 + t39756 + t39760 - t48252 + t46996 - t46998 + t48254 - t48256;
    (t48247, t48248, t48249, t48250, t48251, t48252, t48254, t48256, t48257)
}
