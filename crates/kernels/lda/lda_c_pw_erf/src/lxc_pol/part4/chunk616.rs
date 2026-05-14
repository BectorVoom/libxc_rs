//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 616/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk616<F: Float>(t3080: F, t3152: F, t60: F, t40: F, t2849: F, t88: F, t2851: F, t1063: F, t338: F, t38: F, t461: F, t36: F, t1067: F, t391: F, t358: F, t1070: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3153 = t3080 + t3152;
    let t3154 = t60 * t3153;
    let t3155 = t40 * t3154;
    let t3156 = t2849 * t88;
    let t3157 = 24.0 * t3156;
    let t3158 = t2851 * t88;
    let t3160 = t338 * t1063;
    let t3161 = t3160 * t88;
    let t3165 = 1.0 / t38 / t461;
    let t3166 = t36 * t3165;
    let t3167 = t3166 * t88;
    let t3168 = 120.0 * t3167;
    let t3169 = t1067 * t391;
    let t3171 = t1067 * t358;
    let t3173 = t1070 * t391;
    (t3153, t3154, t3155, t3157, t3158, t3160, t3161, t3165, t3166, t3167, t3168, t3169, t3171, t3173)
}
