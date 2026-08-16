//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 775/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk775<F: Float>(t5071: F, t5139: F, t5138: F, t3074: F, t3077: F, t3149: F, t3151: F, t3153: F, t3156: F, t3158: F, t3165: F, t3182: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5140 = t5139 * t5071;
    let t5142 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t5138 * t5140;
    let t5143 = t3074 / F::cast_from(45.0_f64);
    let t5144 = t3077 / F::cast_from(45.0_f64);
    let t5145 = t3149 / F::cast_from(45.0_f64);
    let t5146 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t3151;
    let t5147 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t3153;
    let t5148 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t3156;
    let t5149 = t3158 / F::cast_from(45.0_f64);
    let t5150 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t3165;
    let t5151 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t3182;
    (t5140, t5142, t5143, t5144, t5145, t5146, t5147, t5148, t5149, t5150, t5151)
}
