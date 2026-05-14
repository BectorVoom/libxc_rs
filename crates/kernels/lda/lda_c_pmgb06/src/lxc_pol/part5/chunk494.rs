//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 494/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk494<F: Float>(t5: F, t153: F, t2582: F, t137: F, t132: F, t2377: F, t2381: F, t44: F, t131: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t2583 = t2582 * t153;
    let t2584 = t137 * t2583;
    let t2586 = t132 * t2584 / 30.0;
    let t2590 = piecewise3(t6, 0.0, 2.0 * t5 * t2381 + 2.0 * t2377);
    let t2591 = t2590 * t44;
    let t2592 = t2591 * t131;
    (t2583, t2584, t2586, t2591, t2592)
}
