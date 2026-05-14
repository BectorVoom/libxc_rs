//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 724/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk724<F: Float>(t6457: F, t6459: F, t6463: F, t6467: F, t6471: F, t6475: F, t6477: F, t6481: F, t6485: F, t6487: F, t6491: F, t6495: F, t6570: F, t6572: F, t6574: F, t6576: F) -> (F,) {
    let t7254 = t6457 - t6459 - t6463 - t6467 - t6471 + t6475 + t6477 - t6481 - t6485 - t6487 + t6491 + t6495 - t6570 + t6572 - t6574 - t6576;
    (t7254,)
}
