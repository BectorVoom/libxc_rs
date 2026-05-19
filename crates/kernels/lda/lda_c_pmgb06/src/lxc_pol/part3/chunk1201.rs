//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1201/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1201<F: Float>(t12508: F, t12511: F, t12518: F, t12524: F, t12527: F, t12534: F, t12542: F, t12545: F, t12550: F, t12553: F, t12557: F, t10691: F, t10693: F, t10696: F, t10697: F, t10699: F, t12561: F, t12566: F, t12571: F, t12574: F, t12576: F, t12579: F, t12583: F) -> (F, F) {
    let t14373 = -t12508 - t12511 - t12518 + t12524 - t12527 - t12534 + t12542 - t12545 + t12550 + t12553 + t12557;
    let t14378 = F::new(4.0) / F::new(3.0) * t10691 + F::cast_from(0.0033101111111111113_f64) * t10693 + t10696 + F::new(8.0) * t10697 + F::new(12.0) * t10699 + t12561 - t12566 + t12571 + t12574 + t12576 + t12579 + t12583;
    (t14373, t14378)
}
