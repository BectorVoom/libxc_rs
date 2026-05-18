//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 954/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk954<F: Float>(t2676: F, t27: F, t545: F, t3007: F, t3026: F, t3028: F, t5104: F, t5107: F, t5114: F, t6430: F, t6433: F, t6434: F, t6440: F, t6445: F, t6447: F, t6451: F, t6453: F) -> (F, F) {
    let t7193 = t2676 * t27;
    let t7194 = t7193 * t545;
    let t7196 = -t6430 - t6433 - t6434 + t3007 + t6440 + t6445 + t6447 + t6451 - t6453 + t3026 + F::new(4.0) / F::new(3.0) * t3028 + F::new(0.10821041362364843) * t7194 - t5104 - t5107 - t5114;
    (t7193, t7196)
}
