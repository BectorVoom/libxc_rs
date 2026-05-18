//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 716/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk716<F: Float>(t2746: F, t298: F, t301: F, t305: F, t8113: F, t19: F, t7380: F, t123: F, t3906: F) -> (F, F, F, F, F, F, F) {
    let t8124 = F::new(1.0) / t2746 / t298;
    let t8125 = t8124 * t301;
    let t8126 = t8125 * t305;
    let t8127 = t8126 * t8113;
    let t8128 = t7380 * t19;
    let t8129 = t8128 * t123;
    let t8134 = t3906 * t8113;
    (t8124, t8125, t8126, t8127, t8128, t8129, t8134)
}
