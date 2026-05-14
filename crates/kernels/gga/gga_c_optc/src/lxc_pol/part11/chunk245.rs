//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 245/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk245<F: Float>(t765: F, t248: F, t243: F, t792: F, t251: F) -> (F, F, F, F, F, F, F) {
    let t803 = 0.17123333333333333333e-1 * t765;
    let t808 = t248 * t248;
    let t809 = 1.0 / t808;
    let t810 = t243 * t809;
    let t812 = 0.516475e0 * t765;
    let t815 = 0.104195e0 * t792;
    let t818 = 1.0 / t251;
    (t803, t808, t809, t810, t812, t815, t818)
}
