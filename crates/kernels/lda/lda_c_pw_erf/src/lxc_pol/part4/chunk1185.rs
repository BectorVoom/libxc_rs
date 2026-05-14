//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1185/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1185<F: Float>(t12984: F, t12987: F, t2146: F, t5322: F, t2163: F, t5334: F, t4763: F, t5317: F, t17475: F, t17479: F, t17483: F, t17488: F, t17491: F, t17495: F, t17497: F, t17499: F, t17503: F, t17506: F, t17509: F, t17514: F) -> (F, F, F, F, F, F) {
    let t17515 = 64.0 / 405.0 * t12984;
    let t17516 = 256.0 / 135.0 * t12987;
    let t17518 = 16.0 / 15.0 * t2146 * t5322;
    let t17520 = 16.0 / 15.0 * t5334 * t2163;
    let t17522 = 16.0 / 15.0 * t4763 * t5317;
    let t17523 = -t17475 + t17479 + t17483 + t17488 - t17491 + t17495 - t17497 + t17499 + t17503 - t17506 + t17509 + t17514 + t17515 + t17516 + t17518 + t17520 - t17522;
    (t17515, t17516, t17518, t17520, t17522, t17523)
}
