//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 789/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk789<F: Float>(t493: F, t5454: F, t4161: F, t4162: F, t4165: F, t5393: F, t5396: F, t5398: F, t5419: F, t5434: F, t5436: F, t5438: F, t5440: F, t5444: F, t5446: F, t5450: F, t5453: F) -> (F, F) {
    let t5456 = F::new(2.0) / F::new(9.0) * t493 * t5454;
    let t5457 = t5393 - t4161 + F::cast_from(0.06649088888888889_f64) * t4162 + t4165 + t5396 + t5398 + t5419 + t5434 + t5436 + t5438 + t5440 + t5444 - t5446 - t5450 - t5453 - t5456;
    (t5456, t5457)
}
