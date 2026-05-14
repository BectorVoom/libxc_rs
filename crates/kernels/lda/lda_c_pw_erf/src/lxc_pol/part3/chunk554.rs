//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 554/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk554<F: Float>(t128: F, t3251: F, t10: F, t1686: F, t19: F, t436: F, t299: F, t411: F, t732: F, t155: F, t1568: F, t119: F, t1691: F, t120: F, t1652: F, t1657: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3252 = t128 * t3251;
    let t3253 = t10 * t3252;
    let t3257 = t1686 * t436 * t19;
    let t3259 = t732 * t299 * t411;
    let t3260 = t3257 * t3259;
    let t3262 = t155 * t1568;
    let t3263 = t119 * t3262;
    let t3264 = t1691 * t3263;
    let t3267 = t1652 * t120 * t19;
    let t3268 = t3267 * t3259;
    let t3269 = 0.9743416666666667 * t3268;
    let t3270 = t1657 * t3263;
    (t3252, t3253, t3257, t3260, t3262, t3264, t3267, t3268, t3269, t3270)
}
