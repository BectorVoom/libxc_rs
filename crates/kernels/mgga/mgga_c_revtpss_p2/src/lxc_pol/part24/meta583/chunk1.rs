//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1815/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1815<F: Float>(t48262: F, t39750: F, t39756: F, t39760: F, t39773: F, t39783: F, t46988: F, t46992: F, t46996: F, t46998: F, t47000: F, t47003: F) -> (F, F) {
    let t91966 = F::cast_from(0.23392894490538584828e1_f64) * t48262;
    let t91967 = t46988 + t46992 + t39750 + t39756 + t39760 + t46996 - t46998 - t47000 + t47003 + t39773 - t91966 - t39783;
    (t91966, t91967)
}
