//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1101/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1101<F: Float>(t350: F, t4881: F, t4886: F, t1827: F, t947: F, t1822: F, t4870: F, t4641: F, t4873: F, t4858: F, t138: F, t4922: F, t9175: F) -> (F, F, F, F, F, F, F, F) {
    let t13345 = t350 * t4881;
    let t13347 = t350 * t4886;
    let t13370 = t947 * t1827;
    let t13372 = t947 * t1822;
    let t13374 = t350 * t4870;
    let t13376 = t4641 * t4873;
    let t13379 = t350 * t4858;
    let t13382 = t138 * t9175 * t4922;
    (t13345, t13347, t13370, t13372, t13374, t13376, t13379, t13382)
}
