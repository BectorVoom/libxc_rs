//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 560/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk560<F: Float>(t2938: F, t497: F, t506: F, t36: F, t2900: F, t2901: F, t2903: F, t2905: F, t2907: F, t2915: F, t2921: F, t2926: F, t2930: F, t2935: F) -> (F, F, F, F) {
    let t2939 = t497 * t2938;
    let t2940 = t506 * t2939;
    let t2941 = t36 * t2940;
    let t2943 = t2900 + F::cast_from(0.002518888888888889_f64) * t2901 - F::cast_from(0.0012594444444444445_f64) * t2903 + F::cast_from(0.003778333333333333_f64) * t2905 - F::cast_from(0.0018891666666666666_f64) * t2907 + F::cast_from(0.002099074074074074_f64) * t2915 - F::cast_from(0.007556666666666666_f64) * t2921 + F::cast_from(0.003778333333333333_f64) * t2926 + F::new(0.011335) * t2930 - F::new(0.011335) * t2935 + F::cast_from(0.0018891666666666666_f64) * t2941;
    (t2939, t2940, t2941, t2943)
}
