//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1187/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1187<F: Float>(t2672: F, t24567: F, t24565: F, t2748: F, t7380: F, t7448: F, t946: F, t24502: F, t330: F, t7453: F, t7837: F, t874: F, t888: F) -> (F, F, F, F, F, F) {
    let t24568 = t2672 * t2672;
    let t24569 = t24567 * t24568;
    let t24574 = t2748 * t24565;
    let t24575 = t24567 * t7380;
    let t24580 = t946 * t7448;
    let t24583 = t330 * t24502;
    let t24584 = t24583 * t7453;
    let t24594 = t874 * t888 * t7837;
    (t24569, t24574, t24575, t24580, t24584, t24594)
}
