//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 573/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk573<F: Float>(t132: F, t3055: F, t1540: F, t464: F, t477: F, t137: F, t188: F, t3007: F, t3009: F, t3014: F, t3015: F, t3019: F, t3026: F, t3028: F, t3037: F, t3042: F, t3045: F, t3049: F, t3052: F, t3054: F) -> (F, F, F, F, F, F, F) {
    let t3056 = t132 * t3055;
    let t3057 = t3056 / F::new(45.0);
    let t3058 = t1540 * t464;
    let t3059 = t3058 * t477;
    let t3060 = t137 * t3059;
    let t3062 = t132 * t3060 / F::new(10.0);
    let t3063 = t3007 - t3009 + t3014 + F::new(4.0) / F::new(3.0) * t3015 * t188 + F::new(4.0) * t3019 + t3026 + F::new(4.0) * t3028 - t3037 + t3042 + t3045 - t3049 - t3052 - t3054 + t3057 - t3062;
    (t3056, t3057, t3058, t3059, t3060, t3062, t3063)
}
