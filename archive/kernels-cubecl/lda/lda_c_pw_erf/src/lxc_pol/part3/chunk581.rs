//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 581/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk581<F: Float>(t145: F, t169: F, t242: F, t2880: F, t2883: F, t2887: F, t2890: F, t2893: F, t2897: F, t2898: F, t2903: F, t2906: F, t2932: F, t2934: F, t2935: F, t2937: F, t3196: F) -> F {
    let t3199 = t2880 - F::cast_from(0.42447554366239165_f64) * t2883 - t2887 + F::cast_from(0.15917832887339686_f64) * t2890 + F::cast_from(0.3183566577467937_f64) * t2893 + t2897 - F::cast_from(0.031835665774679375_f64) * t169 * t2898 * t242 - F::cast_from(0.09550699732403813_f64) * t2903 - F::cast_from(0.09550699732403813_f64) * t2906 - t2932 - t2934 + F::cast_from(0.9598512193592288_f64) * t2935 - F::cast_from(0.31995040645307626_f64) * t2937 + F::cast_from(0.05332506774217938_f64) * t145 * t3196;
    t3199
}
