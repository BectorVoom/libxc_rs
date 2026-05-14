//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1239/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1239<F: Float>(t18409: F, t2171: F, t5299: F, t5238: F, t4738: F, t5409: F, t2146: F, t5286: F, t5279: F, t1472: F, t6272: F, t1403: F, t2415: F, t3867: F, t571: F, t1319: F, t16274: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18410 = 64.0 / 135.0 * t18409;
    let t18412 = 16.0 / 15.0 * t2171 * t5299;
    let t18413 = t2171 * t5238;
    let t18414 = 32.0 / 81.0 * t18413;
    let t18415 = t4738 * t5409;
    let t18416 = 64.0 / 135.0 * t18415;
    let t18418 = 16.0 / 45.0 * t2146 * t5286;
    let t18420 = 16.0 / 15.0 * t2146 * t5279;
    let t18422 = 16.0 / 45.0 * t1472 * t6272;
    let t18426 = 16.0 / 45.0 * t571 * t3867 * t2415 * t1403;
    let t18429 = 32.0 / 15.0 * t571 * t1319 * t16274;
    (t18410, t18412, t18414, t18416, t18418, t18420, t18422, t18426, t18429)
}
