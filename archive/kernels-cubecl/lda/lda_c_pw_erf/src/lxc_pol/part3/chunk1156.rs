//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1156/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1156<F: Float>(t12064: F, t4523: F, t2137: F, t4073: F, t3445: F, t822: F, t2120: F, t3387: F, t2072: F, t5045: F, t2076: F, t3390: F) -> (F, F, F, F, F, F) {
    let t13538 = t12064 * t4523;
    let t13539 = F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t13538;
    let t13540 = t4073 * t2137;
    let t13541 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t13540;
    let t13542 = t822 * t3445;
    let t13543 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t13542;
    let t13544 = t2120 * t3387;
    let t13545 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t13544;
    let t13547 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t5045 * t2072;
    let t13548 = t2076 * t3390;
    (t13539, t13541, t13543, t13545, t13547, t13548)
}
