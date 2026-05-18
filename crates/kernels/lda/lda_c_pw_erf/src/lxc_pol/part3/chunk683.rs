//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 683/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk683<F: Float>(t474: F, t603: F, t602: F, t1: F, t1112: F, t3: F, t604: F, t1631: F, t1635: F, t1422: F, t20: F, t1639: F) -> (F, F, F, F, F, F, F) {
    let t4183 = t474 * t603;
    let t4185 = F::new(0.09618703433213194) * t602 * t4183;
    let t4187 = t1112 * t1 * t3;
    let t4188 = t4187 * t604;
    let t4190 = t1631 * t1635;
    let t4192 = t1422 * t20;
    let t4193 = t4192 * t1639;
    (t4183, t4185, t4187, t4188, t4190, t4192, t4193)
}
