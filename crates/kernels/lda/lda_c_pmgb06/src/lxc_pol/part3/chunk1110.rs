//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1110/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1110<F: Float>(t13201: F, t13178: F, t13181: F, t13185: F, t13187: F, t13189: F, t13191: F, t13193: F, t13195: F, t13197: F, t13200: F, t806: F, t9836: F) -> (F, F, F) {
    let t13202 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t13201;
    let t13203 = t13178 + t13181 + t13185 + t13187 + t13189 - t13191 - t13193 - t13195 - t13197 + t13200 + t13202;
    let t13204 = t9836 * t806;
    (t13202, t13203, t13204)
}
