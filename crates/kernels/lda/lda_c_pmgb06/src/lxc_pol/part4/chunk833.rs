//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 833/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk833<F: Float>(t187: F, t2342: F, t2345: F, t3311: F, t3313: F, t3316: F, t3320: F, t3324: F, t3327: F, t3328: F, t3331: F, t3335: F, t3387: F, t5361: F, t5363: F, t5367: F) -> (F, F, F) {
    let t5674 = F::new(8.0) / F::new(3.0) * t2342 * t187;
    let t5675 = t2345 * t187;
    let t5682 = t5361 - t5363 + t5367 + t5674 + F::new(8.0) / F::new(3.0) * t5675 - t3311 + F::new(0.10821041362364843) * t3313 + F::new(0.4328416544945937) * t3316 + F::new(0.022363485482220676) * t3320 + t3324 + t3327 + F::new(0.1442805514981979) * t3328 + t3331 - t3335 + F::new(4.0) / F::new(3.0) * t3387;
    (t5674, t5675, t5682)
}
