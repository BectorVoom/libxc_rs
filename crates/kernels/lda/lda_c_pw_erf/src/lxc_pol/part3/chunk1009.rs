//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1009/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1009<F: Float>(t13540: F, t3445: F, t822: F, t2120: F, t3387: F, t2072: F, t5045: F, t2076: F, t3390: F, t4729: F, t511: F, t10409: F, t13524: F, t13528: F, t13531: F, t13534: F, t13537: F, t13539: F) -> (F, F, F, F, F, F, F) {
    let t13541 = 8.0 / 15.0 * t13540;
    let t13542 = t822 * t3445;
    let t13543 = 8.0 / 15.0 * t13542;
    let t13544 = t2120 * t3387;
    let t13545 = 8.0 / 15.0 * t13544;
    let t13547 = 8.0 / 5.0 * t5045 * t2072;
    let t13548 = t2076 * t3390;
    let t13549 = 8.0 / 15.0 * t13548;
    let t13550 = t511 * t4729;
    let t13551 = 4.0 / 45.0 * t13550;
    let t13553 = t13524 - t13528 - t13531 - t13534 - t13537 - t13539 + t13541 + t13543 + t13545 - t13547 + t13549 + t13551 + 12.0 * t10409;
    (t13541, t13543, t13545, t13547, t13549, t13551, t13553)
}
