//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1061/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1061<F: Float>(t16178: F, t12113: F, t19696: F, t19697: F, t19698: F, t19699: F, t19700: F, t19701: F, t19705: F, t19706: F, t19707: F, t19708: F) -> (F, F) {
    let t19709 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t16178;
    let t19710 = -t19696 + t19697 + t19698 - t19699 - t19700 - t19701 + t19705 - t19706 - t19707 + t12113 + t19708 - t19709;
    (t19709, t19710)
}
