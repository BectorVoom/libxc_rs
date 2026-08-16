//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 973/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk973(t5043: f64, t5056: f64, t5158: f64, t5166: f64, t5177: f64, t5193: f64, t5993: f64, t6009: f64, t9623: f64, t9631: f64, t9635: f64, t9742: f64, t9750: f64, t9948: f64, t9952: f64, t9956: f64, t9959: f64) -> f64 {
    let t10385 = 1.4770435158815312_f64 * t9948 + 1.4770435158815312_f64 * t9952 - 1.4770435158815312_f64 * t9956 + 0.9846956772543541_f64 * t9959 - 0.2946275542389858_f64 * t9623 - 0.0982091847463286_f64 * t9631 - 0.2946275542389858_f64 * t9635 - 0.2946275542389858_f64 * t9742 - 0.2946275542389858_f64 * t9750 - 0.2946275542389858_f64 * t5043 - 0.0982091847463286_f64 * t5056 + t5993 - 0.9846956772543541_f64 * t5177 + 0.9846956772543541_f64 * t5193 + t6009 - 2.9540870317630623_f64 * t5158 + 2.9540870317630623_f64 * t5166;
    t10385
}
