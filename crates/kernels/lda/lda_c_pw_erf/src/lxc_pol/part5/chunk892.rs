//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 892/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk892<F: Float>(t13172: F, t1325: F, t6692: F, t1621: F, t2660: F, t529: F, t6590: F, t3787: F, t6998: F, t515: F, t6631: F, t2076: F, t4571: F, t2171: F, t4834: F, t519: F, t6427: F, t9304: F) -> (F, F, F, F, F, F, F, F) {
    let t15966 = t1325 * t13172 * t6692;
    let t15971 = t2660 * t1621;
    let t15975 = t529 * t6590;
    let t15983 = t1325 * t3787 * t6998;
    let t16016 = t6631 * t515;
    let t16024 = t2076 * t4571;
    let t16036 = t2171 * t4834;
    let t16042 = t519 * t9304 * t6427;
    (t15966, t15971, t15975, t15983, t16016, t16024, t16036, t16042)
}
