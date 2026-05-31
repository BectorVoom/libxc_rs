//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1081/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1081<F: Float>(t2140: F, t3727: F, t3416: F, t5363: F, t1334: F, t2151: F, t352: F, t571: F, t3787: F, t4886: F, t519: F, t5367: F) -> (F, F, F, F, F) {
    let t12652 = t3727 * t2140;
    let t12653 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t12652;
    let t12654 = t3416 * t5363;
    let t12655 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t12654;
    let t12659 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t571 * t2151 * t1334 * t352;
    let t12661 = t519 * t3787 * t4886;
    let t12662 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t12661;
    let t12664 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t3416 * t5367;
    (t12653, t12655, t12659, t12662, t12664)
}
