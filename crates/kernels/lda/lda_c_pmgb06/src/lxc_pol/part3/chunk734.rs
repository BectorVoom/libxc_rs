//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 734/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk734<F: Float>(t2085: F, t4913: F, t1832: F, t4641: F, t497: F, t524: F, t165: F, t138: F, t2897: F, t146: F, t2060: F, t2901: F, t2903: F, t2905: F, t2907: F, t3336: F, t3350: F, t3352: F, t3354: F, t3365: F, t3368: F, t3380: F, t4906: F, t4909: F, t4911: F) -> (F, F, F, F, F) {
    let t4914 = t4913 * t2085;
    let t4916 = t4641 * t1832;
    let t4918 = t524 * t497;
    let t4922 = t165 * t497;
    let t4924 = t138 * t2897 * t4922;
    let t4934 = F::cast_from(0.008888888888888889_f64) * t2060 * t4906 - F::cast_from(0.007407407407407408_f64) * t4909 - F::cast_from(0.015996296296296297_f64) * t4911 - F::cast_from(0.057777777777777775_f64) * t4914 - F::cast_from(0.2639388888888889_f64) * t4916 + F::cast_from(0.013333333333333334_f64) * t146 * t3365 * t4918 + F::cast_from(0.07198333333333333_f64) * t4924 - F::cast_from(0.008888888888888889_f64) * t3336 - F::cast_from(0.014814814814814815_f64) * t3350 + F::cast_from(0.0044444444444444444_f64) * t3352 + F::cast_from(0.0014814814814814814_f64) * t3354 - F::cast_from(0.023994444444444443_f64) * t2905 - F::cast_from(0.03199259259259259_f64) * t2901 + F::cast_from(0.011997222222222222_f64) * t2907 + F::cast_from(0.007998148148148148_f64) * t2903 - t3368 - t3380;
    (t4916, t4918, t4922, t4924, t4934)
}
