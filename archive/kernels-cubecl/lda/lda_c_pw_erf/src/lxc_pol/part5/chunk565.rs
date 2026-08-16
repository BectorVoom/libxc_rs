//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 565/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk565<F: Float>(t3080: F, t3152: F, t60: F, t40: F, t2849: F, t88: F, t2851: F, t1063: F, t338: F, t38: F, t461: F, t36: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3153 = t3080 + t3152;
    let t3154 = t60 * t3153;
    let t3155 = t40 * t3154;
    let t3156 = t2849 * t88;
    let t3157 = F::cast_from(24.0_f64) * t3156;
    let t3158 = t2851 * t88;
    let t3159 = F::cast_from(144.0_f64) * t3158;
    let t3160 = t338 * t1063;
    let t3161 = t3160 * t88;
    let t3162 = F::cast_from(240.0_f64) * t3161;
    let t3165 = F::cast_from(1.0_f64) / t38 / t461;
    let t3166 = t36 * t3165;
    (t3153, t3154, t3155, t3156, t3157, t3158, t3159, t3160, t3161, t3162, t3165, t3166)
}
