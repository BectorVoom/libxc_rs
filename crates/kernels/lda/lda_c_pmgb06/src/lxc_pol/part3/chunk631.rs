//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 631/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk631<F: Float>(t1710: F, t604: F, t1980: F, t223: F, t224: F, t3009: F, t3014: F, t3037: F, t3042: F, t3045: F, t3049: F, t3052: F, t3054: F, t3057: F, t3062: F, t3065: F, t3067: F, t3070: F, t4143: F, t4146: F) -> (F, F, F) {
    let t4148 = t604 * t1710;
    let t4151 = 8.0 / 405.0 * t223 * t1980;
    let t4152 = -t3009 + t3014 - t4143 * t224 / 15.0 - 2.0 / 15.0 * t4146 + 2.0 / 45.0 * t4148 - t4151 - t3037 + t3042 + t3045 - t3049 - t3052 - t3054 + t3057 - t3062 - t3065 - t3067 - t3070;
    (t4148, t4151, t4152)
}
