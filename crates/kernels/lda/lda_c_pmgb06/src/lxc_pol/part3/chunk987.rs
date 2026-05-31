//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 987/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk987<F: Float>(t107: F, t1180: F, t2164: F, t2786: F, t902: F, t161: F, t489: F, t5109: F, t3457: F, t842: F, t166: F, t3459: F) -> (F, F, F, F) {
    let t11744 = t107 * t1180 * t2164;
    let t11745 = F::cast_from(3.9861630686838536_f64) * t11744;
    let t11747 = t107 * t2786 * t902;
    let t11750 = t161 * t489 * t5109;
    let t11751 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t11750;
    let t11752 = t842 * t3457;
    let t11756 = t161 * t166 * t11752 * t3459 / F::cast_from(5.0_f64);
    (t11745, t11747, t11751, t11756)
}
