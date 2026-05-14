//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1293/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1293<F: Float>(t6185: F, t668: F, t1063: F, t15180: F, t15193: F, t15355: F, t15381: F, t2268: F, t2274: F, t2602: F, t2604: F, t2606: F, t2608: F, t2849: F, t348: F, t35: F, t352: F, t462: F, t5992: F, t6005: F, t6164: F, t6169: F, t6174: F, t6179: F, t753: F, t754: F, t92: F, t93: F, t940: F, t945: F, t951: F, t954: F) -> (F, F) {
    let t19134 = t6185 * t668;
    let t19176 = -40.0 / 81.0 * t2602 * t940 + 320.0 / 27.0 * t753 * t15355 + 40.0 / 27.0 * t6164 * t945 + 160.0 / 9.0 * t92 * t35 * t1063 + 80.0 / 9.0 * t2268 * t462 - 80.0 / 3.0 * t2268 * t2849 + 40.0 / 27.0 * t2604 * t940 + 40.0 / 9.0 * t92 * t5992 * t348 + 20.0 / 9.0 * t6169 * t945 + t15180 - 40.0 / 81.0 * t2606 * t951 - 320.0 / 27.0 * t754 * t15381 + 40.0 / 27.0 * t6174 * t954 + 160.0 / 9.0 * t93 * t35 * t1063 - 80.0 / 9.0 * t2274 * t462 + 80.0 / 3.0 * t2274 * t2849 + 40.0 / 27.0 * t2608 * t951 + 40.0 / 9.0 * t93 * t6005 * t352 + 20.0 / 9.0 * t6179 * t954 - t15193;
    (t19134, t19176)
}
