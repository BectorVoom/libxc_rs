//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 736/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk736<F: Float>(t3388: F, t9627: F, t9128: F, t916: F, t3392: F, t2405: F, t2982: F, t3391: F, t197: F, t7975: F, t1077: F, t2493: F, t3096: F, t3430: F, t154: F, t7073: F) -> (F, F, F, F, F, F, F, F) {
    let t9628 = t9627 * t3388;
    let t9630 = t916 * t9128;
    let t9631 = t9630 * t3392;
    let t9635 = t2982 * t2405;
    let t9636 = t3391 * t9635;
    let t9638 = t197 * t7975;
    let t9639 = t1077 * t9638;
    let t9641 = t3096 * t2493;
    let t9642 = t3430 * t9641;
    let t9644 = t7073 * t154;
    (t9628, t9631, t9635, t9636, t9638, t9639, t9642, t9644)
}
