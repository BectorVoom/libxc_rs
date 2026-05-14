//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1198/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1198<F: Float>(t2397: F, t3745: F, t10467: F, t2396: F, t519: F, t11983: F, t1318: F, t1403: F, t6242: F, t4763: F, t5282: F, t10654: F, t2384: F, t1472: F, t6380: F, t16447: F, t2017: F, t571: F) -> (F, F, F, F, F, F, F) {
    let t17707 = 16.0 / 45.0 * t3745 * t2397;
    let t17709 = t519 * t10467 * t2396;
    let t17710 = 16.0 / 405.0 * t17709;
    let t17714 = 16.0 / 5.0 * t1318 * t11983 * t6242 * t1403;
    let t17715 = t4763 * t5282;
    let t17716 = 64.0 / 135.0 * t17715;
    let t17718 = t1318 * t10654 * t2384;
    let t17719 = 32.0 / 405.0 * t17718;
    let t17721 = 16.0 / 9.0 * t1472 * t6380;
    let t17724 = 8.0 / 9.0 * t571 * t2017 * t16447;
    (t17707, t17710, t17714, t17716, t17719, t17721, t17724)
}
