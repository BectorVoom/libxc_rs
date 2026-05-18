//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1063/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1063<F: Float>(t4320: F, t909: F, t123: F, t317: F, t4001: F, t902: F, t113: F, t1798: F, t247: F, t301: F, t1147: F, t2164: F) -> (F, F, F, F) {
    let t11617 = t4320 * t909;
    let t11624 = t123 * t4001 * t902 * t317;
    let t11628 = t247 * t1798 * t113 * t301;
    let t11632 = t123 * t1147 * t2164 * t317;
    (t11617, t11624, t11628, t11632)
}
