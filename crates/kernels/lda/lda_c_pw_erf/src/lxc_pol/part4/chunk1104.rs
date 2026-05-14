//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1104/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1104<F: Float>(t2146: F, t5302: F, t5307: F, t11983: F, t11984: F, t2065: F, t571: F, t6365: F, t954: F, t2017: F, t2337: F, t3589: F, t951: F, t4776: F, t3416: F, t6371: F) -> (F, F, F, F, F, F, F, F) {
    let t16092 = t2146 * t5302;
    let t16093 = 32.0 / 135.0 * t16092;
    let t16095 = 16.0 / 45.0 * t2146 * t5307;
    let t16099 = 16.0 / 5.0 * t571 * t11983 * t11984 * t2065;
    let t16100 = t6365 * t954;
    let t16103 = 4.0 / 27.0 * t571 * t2017 * t16100;
    let t16105 = t3589 * t2337 * t951;
    let t16108 = 32.0 / 81.0 * t571 * t4776 * t16105;
    let t16110 = 16.0 / 27.0 * t3416 * t6371;
    (t16093, t16095, t16099, t16100, t16103, t16105, t16108, t16110)
}
