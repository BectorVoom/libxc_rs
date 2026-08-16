//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 889/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk889<F: Float>(t206: F, t208: F, t247: F, t161: F, t3004: F, t512: F, t3005: F, t486: F, t2943: F, t495: F, t224: F, t3133: F) -> (F, F, F, F, F) {
    let t9348 = F::cast_from(0.19208479012345678_f64) * t206 * t247 * t208;
    let t9350 = t161 * t3004 * t512;
    let t9352 = t486 * t3005;
    let t9354 = t495 * t2943;
    let t9365 = t3133 * t224;
    (t9348, t9350, t9352, t9354, t9365)
}
