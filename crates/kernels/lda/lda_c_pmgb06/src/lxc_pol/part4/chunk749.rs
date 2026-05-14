//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 749/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk749<F: Float>(t12: F, t176: F, t5415: F, t166: F, t161: F, t1: F, t337: F, t395: F, t1083: F, t1842: F, t247: F, t764: F, t44: F, t131: F, t178: F, t1848: F, t513: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t13 = t12 <= zeta_threshold;
    let t5416 = t5415 * t176;
    let t5417 = t166 * t5416;
    let t5419 = t161 * t5417 / 30.0;
    let t5422 = t337 * t1;
    let t5423 = t5422 * t395;
    let t5430 = piecewise3(t13, 0.0, 2.0 * t1083 * t764 - 4.0 * t12 * t395 + 12.0 * t1842 * t247 - 8.0 * t5423);
    let t5431 = t5430 * t44;
    let t5432 = t5431 * t131;
    let t5434 = t5432 * t178 / 30.0;
    let t5436 = t1848 * t513 / 15.0;
    (t5416, t5417, t5419, t5423, t5431, t5432, t5434, t5436)
}
