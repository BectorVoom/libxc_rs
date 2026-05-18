//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 282/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk282<F: Float>(t248: F, t283: F, t619: F, t636: F, t640: F, t645: F, t688: F, t695: F, t700: F, t897: F, t898: F) -> F {
    let t902 = t619 + t636 - t640 - t645 + t248 * t898 + t688 + F::new(0.0197516734986138) * t897 * t283 - t695 - t700;
    t902
}
