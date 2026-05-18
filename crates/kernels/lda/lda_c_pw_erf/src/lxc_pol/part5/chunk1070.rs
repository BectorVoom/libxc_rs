//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1070/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1070<F: Float>(t8212: F, t8216: F, t19: F, t729: F, t734: F, t8076: F, t11362: F, t8267: F, t11349: F, t11360: F, t8221: F, t8224: F, t8238: F, t8244: F, t8248: F, t8260: F, t8263: F, t8266: F, t8271: F, t8274: F, t8277: F) -> (F, F, F, F, F) {
    let t20048 = F::new(120.0) * t8212;
    let t20049 = F::new(12.0) * t8216;
    let t20052 = t8076 * t729 * t19 * t734;
    let t20054 = F::new(3.0) * t11362;
    let t20055 = F::new(0.021687161765563047) * t8267;
    let t20056 = -t20048 - t20049 - t11349 - t8221 + t8224 + t8238 - t8244 - F::new(0.41076328840066667) * t20052 - t8248 + t8260 + t11360 + t20054 + t8263 - t8266 - t20055 + t8271 + t8274 - t8277;
    (t20048, t20049, t20054, t20055, t20056)
}
