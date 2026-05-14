//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1082/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1082<F: Float>(t15750: F, t2411: F, t954: F, t1319: F, t571: F, t6383: F, t951: F, t2017: F, t172: F, t184: F, t6629: F, t496: F, t4561: F, t822: F, t5401: F, t835: F) -> (F, F, F, F, F, F, F, F) {
    let t15751 = 32.0 / 81.0 * t15750;
    let t15752 = t2411 * t954;
    let t15755 = 8.0 / 15.0 * t571 * t1319 * t15752;
    let t15756 = t6383 * t951;
    let t15759 = 16.0 / 3.0 * t571 * t2017 * t15756;
    let t15761 = t172 * t6629 * t184;
    let t15763 = 8.0 / 15.0 * t15761 * t496;
    let t15764 = t822 * t4561;
    let t15765 = 8.0 / 135.0 * t15764;
    let t15767 = 8.0 / 15.0 * t5401 * t835;
    (t15751, t15752, t15755, t15756, t15759, t15763, t15765, t15767)
}
