//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1421/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1421<F: Float>(t17005: F, t17007: F, t17009: F, t17012: F, t17013: F, t17014: F, t17015: F, t17017: F, t17018: F, t17020: F, t17252: F, t17253: F, t17254: F, t17255: F, t17256: F) -> F {
    let t18294 = -t17005 + t17007 + t17009 + t17012 + t17013 + t17014 + t17015 + t17017 + t17018 + t17020 - t17252 + t17253 + t17254 + t17255 + t17256;
    t18294
}
