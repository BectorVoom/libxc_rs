//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 505/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk505<F: Float>(t377: F, t783: F, t384: F, t787: F, t2214: F, t69: F, t55: F, t68: F) -> (F, F, F, F) {
    let t2238 = t783 * t377;
    let t2241 = t787 * t384;
    let t2245 = t69 * t2214;
    let t2247 = t68 * t55;
    (t2238, t2241, t2245, t2247)
}
