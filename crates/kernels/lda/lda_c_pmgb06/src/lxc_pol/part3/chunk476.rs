//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 476/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk476<F: Float>(t5: F, t1936: F, t1393: F, t607: F, t883: F, t10: F, t760: F, t1: F, t594: F, t332: F, t395: F, t15: F, t764: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t1937 = t1936 / F::cast_from(45.0_f64);
    let t1938 = t1393 / F::cast_from(45.0_f64);
    let t1939 = t883 * t607;
    let t1941 = t10 * t760;
    let t1944 = t594 * t1;
    let t1948 = piecewise3::<F>(t6, F::cast_from(0.0_f64), F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t1941 * t332 + F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t1944 * t395);
    let t1949 = t15 * t764;
    (t1937, t1938, t1939, t1941, t1944, t1948, t1949)
}
