//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1432/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1432<F: Float>(t607: F, t6355: F, t1710: F, t2519: F, t10343: F, t10346: F, t10348: F, t10350: F, t10353: F, t10356: F, t10358: F, t10362: F, t17766: F, t17767: F, t17768: F, t17769: F, t17772: F) -> F {
    let t18329 = t6355 * t607;
    let t18331 = t2519 * t1710;
    let t18333 = -t17766 + t17767 + t17768 + t17769 + t17772 + t10343 / F::cast_from(3.0_f64) + F::cast_from(0.12155555555555556_f64) * t10346 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t10348 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t10350 - F::cast_from(0.027012345679012346_f64) * t10353 - t10356 - t10358 + t10362 - F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t18329 + F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t18331;
    t18333
}
