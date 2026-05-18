//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1246/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1246<F: Float>(t20741: F, t20745: F, t20746: F, t20748: F, t20759: F, t20762: F, t20764: F, t20767: F, t20768: F, t20773: F, t20778: F, t20780: F) -> F {
    let t22017 = -t20741 - t20745 - t20746 - t20748 + t20759 - t20762 + t20764 + t20767 - t20768 - t20773 - t20778 - t20780;
    t22017
}
