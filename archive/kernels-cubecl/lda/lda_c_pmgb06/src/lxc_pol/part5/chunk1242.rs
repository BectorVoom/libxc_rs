//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1242/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1242<F: Float>(t20545: F, t20548: F, t20551: F, t20554: F, t20557: F, t20560: F, t20563: F, t20569: F, t20572: F, t20575: F, t20577: F, t20579: F) -> F {
    let t22006 = -t20545 + t20548 + t20551 - t20554 + t20557 + t20560 - t20563 + t20569 + t20572 - t20575 + t20577 + t20579;
    t22006
}
