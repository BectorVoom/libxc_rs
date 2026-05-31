//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 571/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk571<F: Float>(t144: F, t3031: F, t1594: F, t477: F, t137: F, t132: F, t1600: F, t511: F, t1602: F, t166: F, t161: F, t1603: F, t489: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3032 = t144 * t3031;
    let t3033 = t1594 * t477;
    let t3034 = t3032 * t3033;
    let t3035 = t137 * t3034;
    let t3037 = t132 * t3035 / F::cast_from(5.0_f64);
    let t3038 = t511 * t1600;
    let t3039 = t3038 * t1602;
    let t3040 = t166 * t3039;
    let t3042 = t161 * t3040 / F::cast_from(5.0_f64);
    let t3043 = t489 * t1603;
    (t3032, t3033, t3034, t3035, t3037, t3038, t3039, t3040, t3042, t3043)
}
