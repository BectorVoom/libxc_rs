//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 977/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk977<F: Float>(t1467: F, t5305: F, t1972: F, t3195: F, t3235: F, t3239: F, t1963: F, t3177: F, t1420: F, t4615: F, t4620: F, t10082: F, t10083: F, t10085: F, t13257: F, t13258: F) -> (F, F, F, F, F, F, F, F) {
    let t13260 = t5305 * t1467 / 9.0;
    let t13262 = t1972 * t3195 / 15.0;
    let t13264 = t1972 * t3235 / 15.0;
    let t13266 = t1972 * t3239 / 9.0;
    let t13268 = t3177 * t1963 / 15.0;
    let t13270 = t1420 * t4615 / 15.0;
    let t13272 = t1420 * t4620 / 9.0;
    let t13273 = t10082 - 2.0 / 45.0 * t10083 + 2.0 / 45.0 * t10085 - t13257 - t13258 + t13260 + t13262 + t13264 + t13266 + t13268 + t13270 + t13272;
    (t13260, t13262, t13264, t13266, t13268, t13270, t13272, t13273)
}
