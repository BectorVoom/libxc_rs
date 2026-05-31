//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 654/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk654<F: Float>(t1580: F, t405: F, t526: F, t955: F, t1583: F, t1577: F, t163: F, t497: F, t147: F) -> (F, F, F, F, F, F) {
    let t3336 = t405 * t1580;
    let t3350 = t955 * t526;
    let t3352 = t405 * t1583;
    let t3354 = t405 * t1577;
    let t3357 = F::cast_from(1.0_f64) / t163 / t497;
    let t3358 = t147 * t3357;
    (t3336, t3350, t3352, t3354, t3357, t3358)
}
