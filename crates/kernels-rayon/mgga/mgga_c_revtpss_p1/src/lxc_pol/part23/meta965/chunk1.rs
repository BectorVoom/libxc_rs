//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3263/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3263(t73374: f64, t46989: f64, t46993: f64, t47005: f64, t39747: f64, t39750: f64, t39756: f64, t39760: f64, t39773: f64, t46988: f64, t46992: f64, t46996: f64, t46998: f64, t47003: f64, t48256: f64, t48259: f64) -> (f64, f64, f64, f64, f64) {
    let t85903 = 12.0_f64 * t73374;
    let t85904 = 0.35089341735807877242e1_f64 * t46989;
    let t85905 = 0.51947577317044391277e2_f64 * t46993;
    let t85906 = 24.0_f64 * t47005;
    let t85907 = -t85903 + t39747 + t46988 + t85904 + t46992 + t39750 + t39756 + t39760 - t85905 + t46996 - t46998 - t48256 + t47003 - t85906 - t48259 + t39773;
    (t85903, t85904, t85905, t85906, t85907)
}
