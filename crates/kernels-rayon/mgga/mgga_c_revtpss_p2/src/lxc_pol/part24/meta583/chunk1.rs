//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1815/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1815(t48262: f64, t39750: f64, t39756: f64, t39760: f64, t39773: f64, t39783: f64, t46988: f64, t46992: f64, t46996: f64, t46998: f64, t47000: f64, t47003: f64) -> (f64, f64) {
    let t91966 = 0.23392894490538584828e1_f64 * t48262;
    let t91967 = t46988 + t46992 + t39750 + t39756 + t39760 + t46996 - t46998 - t47000 + t47003 + t39773 - t91966 - t39783;
    (t91966, t91967)
}
