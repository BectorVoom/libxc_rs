//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1095/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1095<F: Float>(t1992: F, t3745: F, t1308: F, t2967: F, t4818: F, t571: F, t10463: F, t1325: F, t2026: F, t2031: F, t2022: F, t9313: F) -> (F, F, F, F, F) {
    let t12803 = F::new(8.0) / F::new(9.0) * t3745 * t1992;
    let t12807 = F::new(8.0) / F::new(15.0) * t571 * t1308 * t4818 * t2967;
    let t12809 = t1325 * t10463 * t2026;
    let t12810 = F::new(16.0) / F::new(135.0) * t12809;
    let t12812 = F::new(8.0) / F::new(15.0) * t3745 * t2031;
    let t12814 = t571 * t9313 * t2022;
    (t12803, t12807, t12810, t12812, t12814)
}
