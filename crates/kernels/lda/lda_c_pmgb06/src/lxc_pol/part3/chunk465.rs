//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 465/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk465<F: Float>(t5: F, t12: F, t1: F, t594: F, t1941: F, t332: F, t395: F, t15: F, t764: F, t598: F, t337: F, t44: F, t441: F, t813: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t13 = t12 <= zeta_threshold;
    let t1944 = t594 * t1;
    let t1948 = piecewise3(t6, 0.0, 40.0 / 9.0 * t1941 * t332 + 16.0 / 3.0 * t1944 * t395);
    let t1949 = t15 * t764;
    let t1952 = t598 * t1;
    let t1956 = piecewise3(t13, 0.0, 40.0 / 9.0 * t1949 * t337 - 16.0 / 3.0 * t1952 * t395);
    let t1959 = (t1948 / 2.0 + t1956 / 2.0) * t44;
    let t1962 = t441 * t813;
    (t1944, t1949, t1952, t1959, t1962)
}
