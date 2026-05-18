//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 832/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk832<F: Float>(t7544: F, t7549: F, t7553: F, t7557: F, t7561: F, t7565: F, t7569: F, t7571: F, t7573: F, t7576: F, t7579: F, t7583: F, t7587: F, t7589: F, t7620: F, t7630: F) -> F {
    let t7948 = -t7544 + t7549 - t7553 - t7557 + t7561 + t7565 + t7569 + t7571 + t7573 + t7576 + t7579 + t7583 + t7587 + t7589 + t7620 + t7630;
    t7948
}
