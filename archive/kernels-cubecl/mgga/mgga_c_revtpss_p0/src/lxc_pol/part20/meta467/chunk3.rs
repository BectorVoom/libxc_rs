//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1789/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1789<F: Float>(t39747: F, t39750: F, t39756: F, t39760: F, t46980: F, t46982: F, t46984: F, t46988: F, t46990: F, t46992: F, t46994: F, t46996: F, t46998: F, t47000: F, t47003: F) -> F {
    let t47634 = -t46980 - t46982 - t46984 + t39747 + t46988 + t46990 + t46992 + t39750 + t39756 + t39760 - t46994 + t46996 - t46998 - t47000 + t47003;
    t47634
}
