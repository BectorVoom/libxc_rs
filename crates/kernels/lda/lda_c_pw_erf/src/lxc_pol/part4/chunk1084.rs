//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1084/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1084<F: Float>(t325: F, t6561: F, t1245: F, t35: F, t1243: F, t15782: F, t6335: F, t940: F, t11: F, t503: F, t6504: F, t4606: F, t6507: F, t6351: F, t945: F, t2325: F, t9777: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15788 = t325 * t6561;
    let t15790 = t1245 * t35;
    let t15792 = t15782 * t1243 * t15790;
    let t15794 = t6335 * t940;
    let t15796 = t11 * t503 * t15794;
    let t15798 = t325 * t6504;
    let t15800 = t4606 * t6507;
    let t15802 = t6351 * t945;
    let t15804 = t11 * t1243 * t15802;
    let t15807 = t9777 * t2325 * t940;
    (t15788, t15790, t15792, t15794, t15796, t15798, t15800, t15802, t15804, t15807)
}
