//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 975/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk975<F: Float>(t160: F, t658: F, t9598: F, t131: F, t634: F, t2086: F, t130: F, t2029: F, t1: F, t6850: F, t6855: F, t140: F, t6916: F) -> (F, F, F, F, F, F, F) {
    let t9599 = t160 * t658;
    let t9600 = t9598 * t9599;
    let t9641 = t634 * t131;
    let t9642 = t9641 * t9599;
    let t9677 = t160 * t2086;
    let t9678 = t9641 * t9677;
    let t9686 = t130 * t2029;
    let t9742 = t6850 * t1;
    let t9747 = t6855 * t1;
    let t9771 = t6916 * t140;
    (t9600, t9642, t9678, t9686, t9742, t9747, t9771)
}
