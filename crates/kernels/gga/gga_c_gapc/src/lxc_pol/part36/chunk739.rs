//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 739/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk739<F: Float>(t291: F, t5211: F, t7956: F, t9677: F, t3127: F, t3402: F, t7944: F, t3132: F, t7259: F, t2492: F, t2701: F, t646: F, t3343: F, t1026: F, t2787: F, t937: F) -> (F, F, F, F, F) {
    let t9679 = t5211 * t291 * t7956;
    let t9680 = t9677 * t9679;
    let t9682 = t3402 * t3127;
    let t9683 = t9682 * t7944;
    let t9685 = t7259 * t3132;
    let t9686 = t9685 * t7944;
    let t9689 = t646 * t2492 * t2701;
    let t9690 = t3343 * t9689;
    let t9692 = t2787 * t1026;
    let t9693 = t9692 * t937;
    (t9680, t9683, t9686, t9690, t9693)
}
