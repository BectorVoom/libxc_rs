//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1036/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1036<F: Float>(t8279: F, t11390: F, t11318: F, t11323: F, t11355: F, t11374: F, t11380: F, t11388: F, t11393: F, t11396: F, t11398: F, t11401: F, t11402: F, t342: F, t7344: F, t5874: F) -> (F, F, F, F, F, F) {
    let t21399 = 1.9486833333333333 * t8279;
    let t21403 = 4.5469277777777775 * t11390;
    let t21406 = t11318 + t11323 + t11355 - t11374 + t11380 + 6.85552 * t11388 + t21403 + 14.0 / 9.0 * t11393 - t11396 + 5.87616 * t11398 - t11401;
    let t21409 = 3.8973666666666666 * t11402;
    let t21410 = t7344 * t342;
    let t21411 = t5874 * t21410;
    (t21399, t21403, t21406, t21409, t21410, t21411)
}
