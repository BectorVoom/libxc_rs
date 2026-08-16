//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1121/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1121<F: Float>(t41448: F, t41911: F, t43480: F, t89: F, t39370: F, t666: F, t792: F, t10415: F, t9725: F, t10271: F, t41962: F, t295: F, t41446: F) -> (F, F, F, F, F) {
    let t43483 = t89 * t41911 * t43480 * t41448;
    let t43487 = t89 * t666 * t792 * t39370;
    let t43490 = t89 * t9725 * t10415;
    let t43493 = t89 * t41962 * t10271;
    let t43495 = t295 * t41446;
    (t43483, t43487, t43490, t43493, t43495)
}
