//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 669/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk669<F: Float>(t12: F, t1: F, t1219: F, t337: F, t395: F, t1080: F, t1083: F, t2200: F, t2203: F, t247: F, t336: F, t4378: F, t4377: F, zeta_threshold: F) -> (F, F) {
    let t13 = t12 <= zeta_threshold;
    let t4381 = t1219 * t1;
    let t4382 = t337 * t395;
    let t4392 = piecewise3(t13, 0.0, 8.0 / 27.0 * t4378 * t1080 + 8.0 / 9.0 * t4381 * t4382 - 2.0 / 9.0 * t2200 * t1083 - 4.0 / 3.0 * t336 * t395 + 4.0 * t2203 * t247);
    let t4394 = t4377 / 2.0 + t4392 / 2.0;
    (t4382, t4394)
}
