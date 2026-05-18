//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 816/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk816<F: Float>(t3391: F, t3392: F, t3395: F, t5396: F, t5398: F, t5419: F, t5434: F, t5436: F, t5438: F, t5440: F, t5444: F, t5446: F, t5450: F, t5453: F, t5456: F) -> F {
    let t5685 = t3391 + F::new(16.0) / F::new(3.0) * t3392 + t3395 + t5396 + t5398 + t5419 + t5434 + t5436 + t5438 + t5440 + t5444 - t5446 - t5450 - t5453 - t5456;
    t5685
}
