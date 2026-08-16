//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 576/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk576(t3080: f64, t3152: f64, t60: f64, t40: f64, t2849: f64, t88: f64, t2851: f64, t1063: f64, t338: f64, t3016: f64, t3019: f64, t3118: f64, t3121: f64, t3125: f64, t3133: f64, t3139: f64, t3151: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3153 = t3080 + t3152;
    let t3154 = t60 * t3153;
    let t3155 = t40 * t3154;
    let t3156 = t2849 * t88;
    let t3157 = 24.0_f64 * t3156;
    let t3158 = t2851 * t88;
    let t3159 = 144.0_f64 * t3158;
    let t3160 = t338 * t1063;
    let t3161 = t3160 * t88;
    let t3162 = 240.0_f64 * t3161;
    let t3163 = t3016 + t3019 + t3155 + t3118 - t3121 + t3125 + t3133 - t3139 + t3151 + t3157 - t3159 + t3162;
    (t3153, t3154, t3155, t3156, t3157, t3158, t3160, t3161, t3163)
}
