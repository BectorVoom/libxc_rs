//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 643/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk643<F: Float>(t1620: F, t226: F, t603: F, t695: F, t1612: F, t230: F, t598: F, t610: F, t225: F, t2853: F, t611: F, t1621: F, t1953: F, t2061: F, t7: F, t231: F, t4046: F, t4054: F, t4056: F, t4058: F, t4061: F, t4065: F, t4069: F, t4071: F, t4075: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4215 = 4.0 * t226 * t1620;
    let t4217 = 0.0011033703703703704 * t695 * t603;
    let t4218 = t1612 * t230;
    let t4220 = t598 * t610;
    let t4222 = t2853 * t225;
    let t4225 = t1612 * t611;
    let t4227 = t598 * t1621;
    let t4231 = 1.2833333333333334 * t1953 - 20.0 / 27.0 * t2061;
    let t4232 = t4231 * M_PI;
    let t4233 = t4232 * t7;
    let t4235 = 4.0 / 3.0 * t226 * t4233;
    let t4236 = t4046 + t4054 + t4056 + t4058 + t4061 + t4065 + t4069 + t4215 + t4217 + 4.0 * t4218 + 8.0 * t4220 + 4.0 / 3.0 * t4222 * t231 + 4.0 * t4225 + 4.0 * t4227 + t4235 - t4071 + t4075;
    (t4215, t4217, t4218, t4220, t4222, t4225, t4227, t4231, t4232, t4233, t4235, t4236)
}
