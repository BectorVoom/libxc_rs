//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 478/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk478<F: Float>(t12: F, t1949: F, t1952: F, t337: F, t395: F, t1948: F, t44: F, t441: F, t813: F, zeta_threshold: F) -> (F, F) {
    let t13 = t12 <= zeta_threshold;
    let t1956 = piecewise3(t13, 0.0, 40.0 / 9.0 * t1949 * t337 - 16.0 / 3.0 * t1952 * t395);
    let t1959 = (t1948 / 2.0 + t1956 / 2.0) * t44;
    let t1962 = t441 * t813;
    (t1959, t1962)
}
