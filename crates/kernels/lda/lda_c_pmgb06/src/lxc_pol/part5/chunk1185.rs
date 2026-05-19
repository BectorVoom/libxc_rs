//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1185/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1185<F: Float>(t8279: F, t11390: F, t11318: F, t11323: F, t11355: F, t11374: F, t11380: F, t11388: F, t11393: F, t11396: F, t11398: F, t11401: F) -> (F, F, F) {
    let t21399 = F::cast_from(1.9486833333333333_f64) * t8279;
    let t21403 = F::cast_from(4.5469277777777775_f64) * t11390;
    let t21406 = t11318 + t11323 + t11355 - t11374 + t11380 + F::new(6.85552) * t11388 + t21403 + F::new(14.0) / F::new(9.0) * t11393 - t11396 + F::new(5.87616) * t11398 - t11401;
    (t21399, t21403, t21406)
}
