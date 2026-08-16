//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3852/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3852<F: Float>(t39747: F, t39750: F, t39756: F, t39760: F, t46988: F, t46992: F, t46996: F, t46998: F, t73367: F, t73371: F, t73372: F, t73373: F, t73375: F, t73379: F, t73380: F, t73384: F, t73388: F) -> F {
    let t74102 = t73367 - t73371 - t73372 - t73373 - t73375 + t39747 + t46988 + t73379 + t46992 + t39750 + t39756 + t39760 - t73380 + t46996 - t46998 - t73384 + t73388;
    t74102
}
