//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 990/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk990<F: Float>(t1821: F, t2624: F, t5138: F, t17598: F, t1911: F, t5068: F, t1826: F, t2653: F, t20533: F, t20536: F, t20539: F, t20541: F, t20543: F, t20545: F, t20548: F, t20551: F) -> (F, F, F, F, F) {
    let t20554 = t5138 * t2624 * t1821 / 9.0;
    let t20557 = 4.0 / 15.0 * t5068 * t17598 * t1911;
    let t20560 = 4.0 / 15.0 * t5068 * t2653 * t1826;
    let t20563 = 2.0 / 9.0 * t5138 * t2653 * t1821;
    let t20564 = t20533 + t20536 - t20539 + t20541 + t20543 - t20545 + t20548 + t20551 - t20554 + t20557 + t20560 - t20563;
    (t20554, t20557, t20560, t20563, t20564)
}
