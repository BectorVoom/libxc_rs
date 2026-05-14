//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 441/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk441<F: Float>(t12: F, t1: F, t336: F, t2200: F, t337: F, t395: F, t2199: F, zeta_threshold: F) -> (F,) {
    let t13 = t12 <= zeta_threshold;
    let t2203 = t336 * t1;
    let t2207 = piecewise3(t13, 0.0, -2.0 / 9.0 * t2200 * t337 - 4.0 / 3.0 * t2203 * t395);
    let t2209 = t2199 / 2.0 + t2207 / 2.0;
    (t2209,)
}
