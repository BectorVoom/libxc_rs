//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 895/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk895<F: Float>(t3056: F, t2108: F, t802: F, t2654: F, t486: F, t4801: F, t851: F, t166: F, t161: F, t2570: F, t477: F, t2960: F) -> (F, F, F, F, F, F, F, F) {
    let t6455 = t3056 / F::cast_from(135.0_f64);
    let t6457 = t802 * t2108 / F::cast_from(15.0_f64);
    let t6459 = t486 * t2654 / F::cast_from(15.0_f64);
    let t6460 = t4801 * t851;
    let t6461 = t166 * t6460;
    let t6463 = t161 * t6461 / F::cast_from(15.0_f64);
    let t6464 = t2570 * t477;
    let t6465 = t2960 * t6464;
    (t6455, t6457, t6459, t6460, t6461, t6463, t6464, t6465)
}
