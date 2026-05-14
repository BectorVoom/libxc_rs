//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1065/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1065<F: Float>(t2660: F, t610: F, t12646: F, t2151: F, t571: F, t833: F, t2562: F, t3742: F, t1392: F, t1440: F, t2471: F, t519: F, t9223: F, t11677: F, t11680: F, t2171: F, t4887: F) -> (F, F, F, F, F, F, F) {
    let t15501 = t2660 * t610;
    let t15506 = 16.0 / 45.0 * t571 * t2151 * t12646 * t833;
    let t15508 = 16.0 / 45.0 * t3742 * t2562;
    let t15513 = 16.0 / 5.0 * t519 * t1440 * t9223 * t2471 * t1392;
    let t15514 = 32.0 / 135.0 * t11677;
    let t15515 = 16.0 / 45.0 * t11680;
    let t15517 = 16.0 / 15.0 * t2171 * t4887;
    (t15501, t15506, t15508, t15513, t15514, t15515, t15517)
}
