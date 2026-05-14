//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 943/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk943<F: Float>(t4137: F, t479: F, t164: F, t8832: F, t163: F, t1645: F, t169: F, t717: F, t1159: F, t1590: F, t695: F, t4100: F, t4120: F, t1198: F, t4263: F, t458: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9180 = 0.0004746123948660562 * t4137 * t479;
    let t9181 = t8832 * t164;
    let t9190 = t169 * t717 * t1645 * t163;
    let t9192 = t1159 * t479;
    let t9195 = 0.3780648866776934 * t695 * t1590;
    let t9199 = t4100 * t479;
    let t9201 = t4120 * t164;
    let t9203 = t1198 * t1590;
    let t9206 = 0.12602162889256446 * t458 * t4263;
    (t9180, t9181, t9190, t9192, t9195, t9199, t9201, t9203, t9206)
}
