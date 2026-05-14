//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 988/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk988<F: Float>(t10758: F, t41454: F, t446: F, t10266: F, t10388: F, t193: F, t89: F, t295: F, t41536: F, t41448: F, t41911: F, t39370: F, t666: F, t792: F, t10415: F, t9725: F) -> (F, F, F, F, F) {
    let t43474 = t446 * t10758 * t41454;
    let t43478 = t89 * t193 * t10266 * t10388;
    let t43480 = t295 * t41536;
    let t43483 = t89 * t41911 * t43480 * t41448;
    let t43487 = t89 * t666 * t792 * t39370;
    let t43490 = t89 * t9725 * t10415;
    (t43474, t43478, t43483, t43487, t43490)
}
