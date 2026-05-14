//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1031/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1031<F: Float>(t10472: F, t10474: F, t10476: F, t10479: F, t10481: F, t10484: F, t10487: F, t10490: F, t10492: F, t10494: F, t9066: F, t9070: F, t10548: F, t789: F, t421: F, t5900: F) -> (F, F, F) {
    let t14257 = -13.28721022894618 * t9066 + t9070 - t10472 + 3.9861630686838536 * t10474 + 0.0837628205355044 * t10476 - t10479 + 0.019897291109174608 * t10481 - 0.5694518669548363 * t10484 + 0.5836538725357885 * t10487 - 0.15917832887339686 * t10490 + 1.5077307696390791 * t10492 + 1.5077307696390791 * t10494;
    let t14270 = t789 * t10548;
    let t14275 = t5900 * t421;
    (t14257, t14270, t14275)
}
