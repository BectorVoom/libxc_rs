//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1244/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1244<F: Float>(t20612: F, t20614: F, t20616: F, t20618: F, t20620: F, t20622: F, t20627: F, t20632: F, t20636: F, t20641: F, t20643: F, t20646: F) -> F {
    let t22009 = t20612 + t20614 + t20616 + t20618 + t20620 + t20622 + t20627 + t20632 - t20636 + t20641 - t20643 - t20646;
    t22009
}
