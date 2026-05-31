//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2696/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2696<F: Float>(t5778: F, t9593: F, t39750: F, t39756: F, t39760: F, t4144: F, t46992: F, t46996: F, t46998: F, t47003: F, t48252: F, t48254: F, t48256: F, t5541: F) -> F {
    let t49575 = t5778 * t9593;
    let t49579 = F::cast_from(6.0_f64) * t4144 * t49575 * t5541 + t39750 + t39756 + t39760 + t46992 + t46996 - t46998 + t47003 - t48252 + t48254 - t48256;
    t49579
}
