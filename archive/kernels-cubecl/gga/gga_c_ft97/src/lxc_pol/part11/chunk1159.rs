//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1159/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1159<F: Float>(t10799: F, t2749: F, t10518: F, t1882: F, t10719: F, t10709: F, t10724: F, t10542: F, t10478: F, t871: F, t2770: F, t2843: F) -> (F, F, F, F, F, F, F, F) {
    let t44483 = t2749 * t10799;
    let t44491 = t1882 * t10518;
    let t44493 = t1882 * t10719;
    let t44495 = t1882 * t10709;
    let t44497 = t1882 * t10724;
    let t44499 = t1882 * t10542;
    let t44518 = t10478 * t871;
    let t44523 = t2770 * t2843;
    (t44483, t44491, t44493, t44495, t44497, t44499, t44518, t44523)
}
