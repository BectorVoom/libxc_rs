//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 979/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk979<F: Float>(t1193: F, t4001: F, t4299: F, t794: F, t4320: F, t909: F, t123: F, t317: F, t902: F, t113: F, t1798: F, t247: F, t301: F) -> (F, F, F, F) {
    let t11615 = t4001 * t794 * t1193 * t4299;
    let t11617 = t4320 * t909;
    let t11624 = t123 * t4001 * t902 * t317;
    let t11628 = t247 * t1798 * t113 * t301;
    (t11615, t11617, t11624, t11628)
}
