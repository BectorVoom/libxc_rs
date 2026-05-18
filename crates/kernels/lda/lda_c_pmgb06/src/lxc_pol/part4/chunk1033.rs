//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1033/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1033<F: Float>(t2778: F, t301: F, t413: F, t398: F, t642: F, t113: F, t1126: F, t1147: F, t123: F, t317: F, t4001: F, t701: F) -> (F, F, F, F, F) {
    let t10643 = F::new(0.026861343269868797) * t2778 * t413 * t301;
    let t10644 = t642 * t398;
    let t10646 = t10644 * t113 * t301;
    let t10657 = t123 * t1147 * t1126 * t317;
    let t10661 = t123 * t4001 * t701 * t317;
    (t10643, t10644, t10646, t10657, t10661)
}
