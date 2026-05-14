//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 855/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk855<F: Float>(t6141: F, t6262: F, t6306: F, t6368: F, t6422: F, t6431: F, t6441: F, t6468: F, t6521: F, t6568: F, t6580: F, t6607: F, t6655: F, t6728: F, t6780: F, t6909: F) -> (F,) {
    let t6913 = t6141 + t6262 + t6306 + t6368 + t6422 + t6431 + t6441 + t6468 + t6521 + t6568 + t6580 + t6607 + t6655 + t6728 + t6780 + t6909;
    (t6913,)
}
