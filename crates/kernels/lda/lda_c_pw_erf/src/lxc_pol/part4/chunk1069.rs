//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1069/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1069<F: Float>(t518: F, t6787: F, t525: F, t13014: F, t826: F, t2146: F, t5342: F, t2171: F, t5339: F, t1318: F, t2531: F, t9432: F, t1472: F, t6925: F, t2544: F, t3727: F) -> (F, F, F, F, F, F, F) {
    let t15563 = t6787 * t518;
    let t15565 = 16.0 / 45.0 * t15563 * t525;
    let t15567 = 16.0 / 45.0 * t13014 * t826;
    let t15568 = t2146 * t5342;
    let t15569 = 16.0 / 405.0 * t15568;
    let t15570 = t2171 * t5339;
    let t15571 = 16.0 / 405.0 * t15570;
    let t15573 = t1318 * t9432 * t2531;
    let t15574 = 16.0 / 135.0 * t15573;
    let t15576 = 8.0 / 45.0 * t1472 * t6925;
    let t15578 = 4.0 / 27.0 * t3727 * t2544;
    (t15565, t15567, t15569, t15571, t15574, t15576, t15578)
}
