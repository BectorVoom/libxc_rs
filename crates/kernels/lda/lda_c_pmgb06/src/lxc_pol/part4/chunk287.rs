//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 287/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk287<F: Float>(t36: F, t97: F, t941: F, t628: F, t944: F, t569: F, t99: F) -> (F, F, F, F) {
    let t949 = F::new(1.0)/F::sqrt(t36);
    let t950 = t949 * t97;
    let t951 = t950 * t941;
    let t953 = t628 * t944;
    let t955 = t99 * t569;
    (t950, t951, t953, t955)
}
