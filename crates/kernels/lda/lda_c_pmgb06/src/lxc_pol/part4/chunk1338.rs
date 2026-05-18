//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1338/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1338<F: Float>(t13507: F, t10720: F, t10727: F, t10735: F, t17564: F, t17571: F, t17575: F, t17576: F, t17578: F, t17583: F, t17584: F, t17585: F, t17587: F, t17588: F, t17589: F) -> (F, F) {
    let t17590 = F::new(4.0) / F::new(81.0) * t13507;
    let t17591 = -t17564 + F::new(8.0) / F::new(3.0) * t10720 + t10727 + F::new(4.0) / F::new(3.0) * t10735 + t17571 - t17575 + t17576 - t17578 - t17583 + t17584 + t17585 + t17587 + t17588 + t17589 + t17590;
    (t17590, t17591)
}
