//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1314/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1314<F: Float>(t17466: F, t17470: F, t17475: F, t17479: F, t17483: F, t17488: F, t17491: F, t17495: F, t17497: F, t17499: F, t17503: F, t17506: F, t17509: F, t17514: F, t17515: F, t17516: F, t17518: F) -> (F,) {
    let t19255 = t17466 + t17470 - t17475 + t17479 + t17483 + t17488 - t17491 + t17495 - t17497 + t17499 + t17503 - t17506 + t17509 + t17514 + t17515 + t17516 + t17518;
    (t19255,)
}
