//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 576/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk576<F: Float>(t1186: F, t485: F, t1124: F, t465: F, t483: F, t1131: F, t1578: F, t1138: F, t1597: F, t2877: F, t474: F, t603: F, t602: F, t1631: F, t1635: F, t1422: F, t20: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4165 = 0.025899545097903542 * t1186 * t485;
    let t4166 = t1124 * t465;
    let t4168 = t4166 * t483 * t485;
    let t4172 = 0.01975389032890948 * t1578 * t1131 * t485;
    let t4175 = 0.0034679929861433484 * t2877 * t1138 * t1597;
    let t4183 = t474 * t603;
    let t4185 = 0.09618703433213194 * t602 * t4183;
    let t4190 = t1631 * t1635;
    let t4192 = t1422 * t20;
    (t4165, t4166, t4168, t4172, t4175, t4183, t4185, t4190, t4192)
}
