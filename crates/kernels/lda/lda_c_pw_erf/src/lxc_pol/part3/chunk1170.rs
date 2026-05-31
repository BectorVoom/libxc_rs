//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1170/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1170<F: Float>(t12445: F, t3965: F, t3967: F, t542: F, t10427: F, t10429: F, t493: F, t9248: F, t10432: F, t10439: F, t3704: F, t4505: F) -> (F, F, F, F, F, F, F) {
    let t13764 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t3965 * t3967 * t12445 * t542;
    let t13765 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t10427;
    let t13766 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t10429;
    let t13767 = t493 * t9248;
    let t13768 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t13767;
    let t13769 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t10432;
    let t13770 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t10439;
    let t13771 = t4505 * t3704;
    (t13764, t13765, t13766, t13768, t13769, t13770, t13771)
}
