//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 336/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk336<F: Float>(t1159: F, t159: F, t285: F, t477: F, t695: F, t684: F, t688: F, t692: F, t1112: F, t465: F, t281: F, t1128: F, t147: F) -> (F, F, F, F, F, F, F, F) {
    let t1161 = t1159 * t159 * t285;
    let t1165 = F::cast_from(0.0005811348303577384_f64) * t695 * t477 * t285;
    let t1166 = t684 * t688;
    let t1169 = F::cast_from(0.039914113367515366_f64) * t684 * t692;
    let t1171 = t1112 * t159 * t285;
    let t1175 = t465 * t477 * t285;
    let t1176 = t281 * t1175;
    let t1179 = t147 * t1128 * t285;
    (t1161, t1165, t1166, t1169, t1171, t1175, t1176, t1179)
}
