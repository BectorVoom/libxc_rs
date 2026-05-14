//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1054/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1054<F: Float>(t462: F, t5718: F, t1891: F, t39: F, t19: F, t5944: F, t729: F, t734: F, t1746: F, t5949: F, t11307: F, t11309: F, t8178: F, t8180: F, t11313: F, t11315: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15272 = t462 * t5718;
    let t15274 = t39 * t1891;
    let t15288 = t5944 * t729 * t19 * t734;
    let t15296 = t5949 * t1746;
    let t15332 = 7.017868076946245 * t11307;
    let t15333 = 103.89453539625518 * t11309;
    let t15334 = 17.315755899375862 * t8178;
    let t15335 = 2050.779404201559 * t8180;
    let t15336 = 0.043374323531126094 * t11313;
    let t15337 = 0.06506148529668915 * t11315;
    (t15272, t15274, t15288, t15296, t15332, t15333, t15334, t15335, t15336, t15337)
}
