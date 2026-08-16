//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 554/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk554<F: Float>(t1179: F, t139: F, t138: F, t163: F, t508: F, t947: F, t1478: F, t350: F, t1482: F, t1486: F, t1461: F, t1463: F, t158: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2897 = t1179 * t139;
    let t2899 = t138 * t2897 * t163;
    let t2900 = F::cast_from(0.005877407407407408_f64) * t2899;
    let t2901 = t947 * t508;
    let t2903 = t350 * t1478;
    let t2905 = t350 * t1482;
    let t2907 = t350 * t1486;
    let t2909 = t139 * t1461;
    let t2911 = F::cast_from(1.0_f64) / t1463 / t158;
    (t2897, t2899, t2900, t2901, t2903, t2905, t2907, t2909, t2911)
}
