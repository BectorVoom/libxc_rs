//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 826/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk826<F: Float>(t2604: F, t3290: F, t137: F, t132: F, t2601: F, t486: F, t2599: F, t3038: F, t166: F, t161: F, t1887: F, t824: F, t3056: F, t2108: F, t802: F, t2654: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6442 = t3290 * t2604;
    let t6443 = t137 * t6442;
    let t6445 = t132 * t6443 / 15.0;
    let t6447 = t486 * t2601 / 15.0;
    let t6448 = t3038 * t2599;
    let t6449 = t166 * t6448;
    let t6451 = t161 * t6449 / 15.0;
    let t6453 = t1887 * t824 / 15.0;
    let t6455 = t3056 / 135.0;
    let t6457 = t802 * t2108 / 15.0;
    let t6459 = t486 * t2654 / 15.0;
    (t6442, t6443, t6445, t6447, t6448, t6449, t6451, t6453, t6455, t6457, t6459)
}
