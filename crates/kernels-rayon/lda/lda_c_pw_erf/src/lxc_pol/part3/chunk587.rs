//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 587/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk587(t1691: f64, t3263: f64, t120: f64, t1652: f64, t19: f64, t3259: f64, t1657: f64, t1: f64, t128: f64, t415: f64, t3212: f64, t3216: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3264 = t1691 * t3263;
    let t3267 = t1652 * t120 * t19;
    let t3268 = t3267 * t3259;
    let t3269 = 0.9743416666666667_f64 * t3268;
    let t3270 = t1657 * t3263;
    let t3271 = 1.4615125_f64 * t3270;
    let t3273 = t415 * t128 * t1;
    let t3274 = t3273 * t3212;
    let t3275 = 2.923025_f64 * t3274;
    let t3276 = t1657 * t3216;
    (t3264, t3267, t3268, t3269, t3270, t3271, t3273, t3274, t3275, t3276)
}
