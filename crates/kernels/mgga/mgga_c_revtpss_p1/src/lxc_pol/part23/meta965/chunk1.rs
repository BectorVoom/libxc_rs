//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3263/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3263<F: Float>(t73374: F, t46989: F, t46993: F, t47005: F, t39747: F, t39750: F, t39756: F, t39760: F, t39773: F, t46988: F, t46992: F, t46996: F, t46998: F, t47003: F, t48256: F, t48259: F) -> (F, F, F, F, F) {
    let t85903 = F::cast_from(12.0_f64) * t73374;
    let t85904 = F::cast_from(0.35089341735807877242e1_f64) * t46989;
    let t85905 = F::cast_from(0.51947577317044391277e2_f64) * t46993;
    let t85906 = F::cast_from(24.0_f64) * t47005;
    let t85907 = -t85903 + t39747 + t46988 + t85904 + t46992 + t39750 + t39756 + t39760 - t85905 + t46996 - t46998 - t48256 + t47003 - t85906 - t48259 + t39773;
    (t85903, t85904, t85905, t85906, t85907)
}
