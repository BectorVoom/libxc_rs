//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 555/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk555<F: Float>(t3270: F, t1: F, t128: F, t415: F, t3212: F, t1657: F, t3216: F, t3213: F, t3217: F, t3220: F, t3224: F, t3228: F, t3231: F, t3253: F, t3260: F, t3264: F, t3269: F, t426: F) -> (F, F, F, F, F, F, F) {
    let t3271 = 1.4615125 * t3270;
    let t3273 = t415 * t128 * t1;
    let t3274 = t3273 * t3212;
    let t3275 = 2.923025 * t3274;
    let t3276 = t1657 * t3216;
    let t3277 = 1.9486833333333333 * t3276;
    let t3278 = -8.81424 * t3213 - 2.93808 * t3217 - 3.0 / 2.0 * t3220 - 6.0 * t426 * t3224 - 2.0 / 3.0 * t3228 + t3231 / 2.0 - t426 * t3253 / 2.0 - 1.46904 * t3260 + 2.20356 * t3264 + t3269 + t3271 - t3275 - t3277;
    (t3271, t3273, t3274, t3275, t3276, t3277, t3278)
}
