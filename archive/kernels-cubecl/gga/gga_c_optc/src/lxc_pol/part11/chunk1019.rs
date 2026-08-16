//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1019/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1019<F: Float>(t22626: F, t539: F, t6525: F, t1860: F, t1993: F, t601: F, t1864: F, t1867: F, t22075: F, t592: F, t6326: F, t6322: F) -> (F, F, F, F, F, F) {
    let t22627 = F::cast_from(384.0_f64) * t22626;
    let t22635 = t539 * t6525;
    let t22636 = F::cast_from(16.0_f64) * t22635;
    let t22641 = F::cast_from(0.21053604230838734656e2_f64) * t601 * t1993 * t1860;
    let t22652 = F::cast_from(0.51947267698127589897e2_f64) * t601 * t1864 * t22075 * t1867;
    let t22655 = F::cast_from(480.0_f64) * t6326 * t592;
    let t22656 = t6322 * t592;
    (t22627, t22636, t22641, t22652, t22655, t22656)
}
