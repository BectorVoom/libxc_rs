//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 821/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk821<F: Float>(t1159: F, t479: F, t1590: F, t695: F, t1198: F, t4263: F, t458: F, t1191: F, t163: F, t169: F, t616: F, t196: F, t3674: F, t218: F, t3666: F, t3437: F, t565: F) -> (F, F, F, F, F, F, F, F) {
    let t9192 = t1159 * t479;
    let t9195 = 0.3780648866776934 * t695 * t1590;
    let t9203 = t1198 * t1590;
    let t9206 = 0.12602162889256446 * t458 * t4263;
    let t9211 = t169 * t1191 * t616 * t163;
    let t9223 = 1.0 / t3674 / t196;
    let t9237 = 1.0 / t3666 / t218;
    let t9246 = t565 * t3437;
    (t9192, t9195, t9203, t9206, t9211, t9223, t9237, t9246)
}
