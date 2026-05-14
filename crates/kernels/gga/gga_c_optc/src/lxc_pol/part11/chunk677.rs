//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 677/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk677<F: Float>(t2663: F, t910: F, t140: F, t305: F, t329: F, t2748: F, t2672: F, t297: F) -> (F, F, F, F, F) {
    let t7369 = 1.0 / t2663 / t910;
    let t7371 = t305 * t7369 * t140;
    let t7372 = t329 * t7371;
    let t7379 = t2748 * t7371;
    let t7380 = t2672 * t297;
    (t7369, t7371, t7372, t7379, t7380)
}
