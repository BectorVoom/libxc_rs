//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 854/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk854<F: Float>(t1882: F, t21488: F, t21639: F, t761: F, t21748: F, t8392: F, t21492: F, t21719: F, t681: F, t89: F, t21761: F, t21674: F, t21754: F, t21537: F, t21416: F, t258: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t80316 = t1882 * t21488;
    let t80334 = t761 * t21639;
    let t80345 = t8392 * t21748;
    let t80399 = t1882 * t21492;
    let t80406 = t89 * t681 * t21719;
    let t80412 = t8392 * t21761;
    let t80429 = t8392 * t21674;
    let t80431 = t8392 * t21754;
    let t80433 = t1882 * t21537;
    let t80460 = t258 * t21416;
    (t80316, t80334, t80345, t80399, t80406, t80412, t80429, t80431, t80433, t80460)
}
