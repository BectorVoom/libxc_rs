//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 673/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk673<F: Float>(t3434: F, t9116: F, t2221: F, t1580: F, t558: F, t2222: F, t609: F, t2211: F, t2210: F, t8376: F, t3440: F, t3439: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9344 = t3434 * t9116;
    let t9345 = t2221 * t9344;
    let t9348 = t1580 * t558;
    let t9349 = t2222 * t9348;
    let t9350 = t2221 * t9349;
    let t9353 = t1580 * t609;
    let t9354 = t2211 * t9353;
    let t9355 = t2210 * t9354;
    let t9358 = t3434 * t8376;
    let t9359 = t2210 * t9358;
    let t9362 = t3440 * t8376;
    let t9363 = t3439 * t9362;
    (t9344, t9345, t9348, t9349, t9350, t9354, t9355, t9358, t9359, t9362, t9363)
}
