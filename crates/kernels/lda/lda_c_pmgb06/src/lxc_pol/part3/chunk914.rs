//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 914/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk914<F: Float>(t12: F, t12274: F, t1504: F, t1848: F, t3073: F, t831: F, t132: F, t435: F, t4681: F, t1842: F, t642: F, t1: F, t1083: F, t247: F, t2938: F, t395: F, t4382: F, t5422: F, t764: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t13 = t12 <= zeta_threshold;
    let t12275 = t12274 / 15.0;
    let t12276 = t1848 * t1504;
    let t12277 = 2.0 / 15.0 * t12276;
    let t12278 = t831 * t3073;
    let t12279 = t12278 / 15.0;
    let t12281 = t132 * t435 * t4681;
    let t12282 = t12281 / 15.0;
    let t12294 = 48.0 * t1842 * t642;
    let t12296 = piecewise3(t13, 0.0, -12.0 * t1083 * t1 * t395 + 24.0 * t12 * t247 + 36.0 * t5422 * t247 + 2.0 * t2938 * t764 - t12294 - 12.0 * t4382);
    (t12275, t12277, t12279, t12282, t12296)
}
