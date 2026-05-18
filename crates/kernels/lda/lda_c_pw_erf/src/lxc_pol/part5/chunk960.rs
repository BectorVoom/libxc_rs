//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 960/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk960<F: Float>(t10463: F, t1325: F, t2006: F, t1896: F, t603: F, t1440: F, t3675: F, t1390: F, t3787: F, t2026: F, t2022: F, t571: F, t9313: F) -> (F, F, F, F, F, F) {
    let t12708 = t1325 * t10463 * t2006;
    let t12709 = F::new(16.0) / F::new(135.0) * t12708;
    let t12714 = t1896 * t603;
    let t12765 = t1440 * t3675;
    let t12781 = t3787 * t1390;
    let t12809 = t1325 * t10463 * t2026;
    let t12810 = F::new(16.0) / F::new(135.0) * t12809;
    let t12814 = t571 * t9313 * t2022;
    (t12709, t12714, t12765, t12781, t12810, t12814)
}
