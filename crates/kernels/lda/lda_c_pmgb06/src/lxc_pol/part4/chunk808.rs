//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 808/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk808<F: Float>(t4885: F, t5470: F, t1919: F, t4866: F, t1924: F, t2979: F, t1962: F, t464: F) -> (F, F, F, F) {
    let t5471 = t5470 * t4885;
    let t5474 = t1919 * t4866;
    let t5477 = t2979 * t1924;
    let t5482 = t1962 * t464;
    (t5471, t5474, t5477, t5482)
}
