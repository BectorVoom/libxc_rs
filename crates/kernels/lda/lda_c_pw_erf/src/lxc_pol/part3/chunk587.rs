//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 587/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk587<F: Float>(t1691: F, t3263: F, t120: F, t1652: F, t19: F, t3259: F, t1657: F, t1: F, t128: F, t415: F, t3212: F, t3216: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3264 = t1691 * t3263;
    let t3267 = t1652 * t120 * t19;
    let t3268 = t3267 * t3259;
    let t3269 = F::new(0.9743416666666667) * t3268;
    let t3270 = t1657 * t3263;
    let t3271 = F::new(1.4615125) * t3270;
    let t3273 = t415 * t128 * t1;
    let t3274 = t3273 * t3212;
    let t3275 = F::new(2.923025) * t3274;
    let t3276 = t1657 * t3216;
    (t3264, t3267, t3268, t3269, t3270, t3271, t3273, t3274, t3275, t3276)
}
