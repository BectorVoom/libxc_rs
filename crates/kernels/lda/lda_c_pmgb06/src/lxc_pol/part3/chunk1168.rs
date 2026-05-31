//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1168/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1168<F: Float>(t1894: F, t3213: F, t1423: F, t5365: F, t13921: F, t13923: F, t13926: F, t13929: F, t13932: F, t13936: F, t13938: F, t13941: F, t13943: F, t13947: F) -> (F, F, F) {
    let t13948 = t3213 * t1894;
    let t13949 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t13948;
    let t13950 = t1423 * t5365;
    let t13951 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t13950;
    let t13952 = t13921 - t13923 - t13926 - t13929 + t13932 - t13936 + t13938 + t13941 + t13943 - t13947 + t13949 + t13951;
    (t13949, t13951, t13952)
}
