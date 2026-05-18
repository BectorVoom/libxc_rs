//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1177/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1177<F: Float>(t2269: F, t7467: F, t2640: F, t7484: F, t2639: F, t2730: F, t2643: F, t7299: F, t2641: F, t7373: F, t7380: F, t769: F) -> (F, F, F, F, F) {
    let t24416 = t7467 * t2269;
    let t24418 = t2640 * t24416 * t7484;
    let t24420 = t2730 * t2639;
    let t24427 = t2643 * t7299;
    let t24431 = t2641 * t7373;
    let t24432 = t7380 * t769;
    (t24418, t24420, t24427, t24431, t24432)
}
