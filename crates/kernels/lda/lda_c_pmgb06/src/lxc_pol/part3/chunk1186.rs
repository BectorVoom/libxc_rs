//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1186/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1186<F: Float>(t10472: F, t10474: F, t10476: F, t10479: F, t10481: F, t10484: F, t10487: F, t10490: F, t10492: F, t10494: F, t9066: F, t9070: F) -> F {
    let t14257 = -F::cast_from(13.28721022894618_f64) * t9066 + t9070 - t10472 + F::cast_from(3.9861630686838536_f64) * t10474 + F::cast_from(0.0837628205355044_f64) * t10476 - t10479 + F::cast_from(0.019897291109174608_f64) * t10481 - F::cast_from(0.5694518669548363_f64) * t10484 + F::cast_from(0.5836538725357885_f64) * t10487 - F::cast_from(0.15917832887339686_f64) * t10490 + F::cast_from(1.5077307696390791_f64) * t10492 + F::cast_from(1.5077307696390791_f64) * t10494;
    t14257
}
