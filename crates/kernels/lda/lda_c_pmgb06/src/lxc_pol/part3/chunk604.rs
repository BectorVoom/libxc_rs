//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 604/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk604<F: Float>(t3725: F, t696: F, t683: F, t957: F, t978: F, t3662: F, t3672: F, t3678: F, t3700: F, t3701: F, t3707: F, t3713: F, t3714: F, t3719: F, t3721: F, t1179: F, t282: F, t55: F) -> (F, F, F, F, F) {
    let t3727 = 51.94757731704439 * t696 * t3725;
    let t3729 = t978 * t957 * t683;
    let t3731 = 3.5089341735807875 * t696 * t3729;
    let t3732 = 0.0007324578922402618 * t3662 + t3672 - t3678 + t3700 - 1.7544670867903938 * t3701 - t3707 + t3713 + 3.5089341735807875 * t3714 + t3719 - 0.0005493434191801964 * t3721 - t3727 + t3731;
    let t3734 = t55 * t1179 * t282;
    (t3727, t3729, t3731, t3732, t3734)
}
