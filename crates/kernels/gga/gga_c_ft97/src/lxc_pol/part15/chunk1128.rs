//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1128/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1128<F: Float>(t41536: F, t88252: F, t41752: F, t92: F, t668: F, t86571: F, t683: F, t41745: F, t52453: F, t66197: F, t66221: F, t80029: F, t80031: F, t80087: F, t80089: F, t80091: F) -> (F, F, F, F, F) {
    let t88726 = t41536 * t88252;
    let t88728 = t92 * t41752 * t88726;
    let t88730 = t668 * t86571;
    let t88732 = t92 * t683 * t88730;
    let t88734 = -F::new(16.0) / F::new(9.0) * t80087 + F::new(8.0) / F::new(3.0) * t80089 + F::new(112.0) / F::new(81.0) * t52453 + F::new(8.0) / F::new(9.0) * t80029 - F::new(8.0) / F::new(3.0) * t80031 + F::new(40.0) / F::new(81.0) * t80091 + F::new(16.0) / F::new(9.0) * t66221 - F::new(16.0) / F::new(27.0) * t66197 + t41745 - F::new(80.0) / F::new(81.0) * t88728 - t88732 / F::new(3.0);
    (t88726, t88728, t88730, t88732, t88734)
}
