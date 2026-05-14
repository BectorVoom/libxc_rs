//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 560/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk560<F: Float>(t5: F, t2501: F, t439: F, t2464: F, t2468: F, t2472: F, t2474: F, t2476: F, t2479: F, t2483: F, t2487: F, t2491: F, t2495: F, t2499: F, t10: F, t2377: F, t2381: F, t594: F, zeta_threshold: F) -> (F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t2503 = 2.0 / 45.0 * t439 * t2501;
    let t2504 = t2464 + t2468 + t2472 + t2474 + t2476 + t2479 + t2483 + t2487 - t2491 - t2495 - t2499 - t2503;
    let t2510 = piecewise3(t6, 0.0, 40.0 / 9.0 * t10 * t2377 + 8.0 / 3.0 * t594 * t2381);
    (t2503, t2504, t2510)
}
