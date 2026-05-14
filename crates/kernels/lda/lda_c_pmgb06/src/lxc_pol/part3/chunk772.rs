//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 772/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk772<F: Float>(t1227: F, t27: F, t402: F, t3494: F, t365: F, t348: F, t361: F, t20: F, t369: F, t3501: F, t3502: F, t642: F, t3509: F, t3510: F, t56: F, t247: F, t28: F, t342: F) -> (F, F, F, F, F, F, F, F) {
    let t8228 = t1227 * t27 * t402;
    let t8229 = t365 * t3494 * t8228;
    let t8232 = t348 * t361 * t8228;
    let t8245 = 1.0 / t369 / t20;
    let t8263 = 15.589466666666667 * t3501 * t3502 * t642;
    let t8266 = 2.6116266666666665 * t3509 * t3510 * t642;
    let t8276 = t3501 * t56;
    let t8278 = t28 * t247 * t342;
    (t8228, t8229, t8232, t8245, t8263, t8266, t8276, t8278)
}
