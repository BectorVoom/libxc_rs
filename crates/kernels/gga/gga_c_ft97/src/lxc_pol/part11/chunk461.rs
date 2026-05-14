//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 461/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk461<F: Float>(t2655: F, t375: F, t825: F, t89: F, t2347: F, t295: F, t2349: F, t2345: F, t683: F, t798: F) -> (F, F, F, F, F, F, F) {
    let t2656 = t2655 / 27.0;
    let t2658 = t89 * t375 * t825;
    let t2659 = t2658 / 9.0;
    let t2660 = t295 * t2347;
    let t2661 = t2660 * t2349;
    let t2663 = t89 * t2345 * t2661;
    let t2665 = t683 * t798;
    (t2656, t2658, t2659, t2660, t2661, t2663, t2665)
}
