//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 564/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk564<F: Float>(t1897: F, t2970: F, t439: F, t1915: F, t2933: F, t493: F, t1387: F, t1420: F, t1450: F, t517: F) -> (F, F, F, F, F, F) {
    let t2971 = t1897 * t2970;
    let t2973 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t439 * t2971;
    let t2974 = t1915 * t2933;
    let t2976 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t493 * t2974;
    let t2978 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t1420 * t1387;
    let t2979 = t1450 * t517;
    (t2971, t2973, t2974, t2976, t2978, t2979)
}
