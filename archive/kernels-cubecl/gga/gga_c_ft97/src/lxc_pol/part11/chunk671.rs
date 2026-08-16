//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 671/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk671<F: Float>(t2180: F, t558: F, t2179: F, t574: F, t2075: F, t609: F, t605: F, t1637: F, t599: F, t89: F, t167: F, t569: F, t7973: F) -> (F, F, F, F, F, F) {
    let t9311 = t2180 * t558;
    let t9313 = t574 * t2179 * t9311;
    let t9316 = t2075 * t609;
    let t9318 = t574 * t605 * t9316;
    let t9321 = t89 * t1637 * t599;
    let t9324 = t569 * t167 * t7973;
    (t9311, t9313, t9316, t9318, t9321, t9324)
}
