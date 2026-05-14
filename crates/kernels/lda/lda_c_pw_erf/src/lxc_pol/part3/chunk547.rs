//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 547/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk547<F: Float>(t3181: F, t1039: F, t344: F, t1037: F, t339: F, t2979: F, t87: F, t40: F, t390: F, t960: F, t3168: F, t3170: F, t3172: F, t3174: F, t3176: F, t3178: F, t3180: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3182 = 24.0 * t3181;
    let t3183 = t344 * t1039;
    let t3184 = 24.0 * t3183;
    let t3185 = t339 * t1037;
    let t3186 = 12.0 * t3185;
    let t3187 = t344 * t1037;
    let t3188 = 12.0 * t3187;
    let t3189 = t2979 * t87;
    let t3190 = t40 * t3189;
    let t3191 = t960 * t390;
    let t3192 = t40 * t3191;
    let t3193 = 3.0 * t3192;
    let t3194 = -t3168 + t3170 + t3172 - t3174 + t3176 - t3178 + t3180 + t3182 - t3184 + t3186 - t3188 + t3190 + t3193;
    (t3182, t3183, t3185, t3187, t3189, t3190, t3191, t3192, t3194)
}
