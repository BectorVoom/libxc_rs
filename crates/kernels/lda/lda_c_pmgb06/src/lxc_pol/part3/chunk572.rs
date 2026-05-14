//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 572/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk572<F: Float>(t144: F, t3259: F, t153: F, t3092: F, t3010: F, t439: F, t1420: F, t1431: F, t3212: F, t3215: F, t3219: F, t3222: F, t3225: F, t3228: F, t3230: F, t3232: F, t3234: F, t3237: F, t3241: F, t3245: F, t3253: F, t3257: F) -> (F, F, F, F, F, F) {
    let t3260 = t3259 * t144;
    let t3261 = t153 * t3092;
    let t3262 = t3261 * t3010;
    let t3263 = t3260 * t3262;
    let t3265 = 8.0 / 81.0 * t439 * t3263;
    let t3267 = t1420 * t1431 / 15.0;
    let t3268 = t3212 - t3215 + t3219 + t3222 - t3225 + t3228 + t3230 + t3232 + t3234 + t3237 + t3241 + t3245 + t3253 + t3257 + t3265 + t3267;
    (t3260, t3262, t3263, t3265, t3267, t3268)
}
