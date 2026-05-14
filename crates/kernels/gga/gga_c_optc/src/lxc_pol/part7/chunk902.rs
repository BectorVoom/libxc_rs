//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 902/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk902<F: Float>(t8593: F, t8595: F, t8598: F, t8601: F, t8603: F, t8606: F, t8609: F, t8651: F, t8654: F, t8657: F, t8660: F, t9279: F, t1199: F, t2879: F, t1196: F, t2885: F) -> (F, F, F) {
    let t9291 = 0.12925555555555555555e1 * t8593 - 0.4926e-2 * t8595 + 0.2463e-2 * t8598 - 0.12315e-2 * t8651 - 0.7389e-2 * t8601 + 0.7389e-2 * t8654 - 0.38776666666666666665e1 * t8603 + 0.77553333333333333331e1 * t8606 - 0.38776666666666666665e1 * t8657 - 0.11633e2 * t8609 + 0.11633e2 * t8660;
    let t9292 = t9279 + t9291;
    let t9294 = t2879 * t1199;
    let t9297 = t1196 * t2885;
    (t9292, t9294, t9297)
}
