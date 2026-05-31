//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 769/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk769<F: Float>(t3836: F, t7359: F, t2608: F, t888: F, t874: F, t2663: F, t910: F, t140: F, t305: F, t329: F, t2670: F, t875: F) -> (F, F, F, F, F, F, F) {
    let t7360 = t3836 * t7359;
    let t7365 = t888 * t2608;
    let t7366 = t874 * t7365;
    let t7369 = F::cast_from(1.0_f64) / t2663 / t910;
    let t7371 = t305 * t7369 * t140;
    let t7372 = t329 * t7371;
    let t7373 = t2670 * t875;
    (t7360, t7365, t7366, t7369, t7371, t7372, t7373)
}
