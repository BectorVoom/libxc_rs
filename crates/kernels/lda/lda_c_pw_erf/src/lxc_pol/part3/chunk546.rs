//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 546/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk546<F: Float>(t38: F, t461: F, t36: F, t88: F, t1067: F, t391: F, t358: F, t1070: F, t1064: F, t1039: F, t339: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3165 = 1.0 / t38 / t461;
    let t3166 = t36 * t3165;
    let t3167 = t3166 * t88;
    let t3168 = 120.0 * t3167;
    let t3169 = t1067 * t391;
    let t3170 = 36.0 * t3169;
    let t3171 = t1067 * t358;
    let t3172 = 36.0 * t3171;
    let t3173 = t1070 * t391;
    let t3174 = 96.0 * t3173;
    let t3175 = t1064 * t391;
    let t3176 = 60.0 * t3175;
    let t3177 = t1070 * t358;
    let t3178 = 96.0 * t3177;
    let t3179 = t1064 * t358;
    let t3180 = 60.0 * t3179;
    let t3181 = t339 * t1039;
    (t3165, t3166, t3167, t3168, t3169, t3170, t3171, t3172, t3173, t3174, t3175, t3176, t3178, t3179, t3180, t3181)
}
