//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 783/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk783<F: Float>(t1041: F, t1043: F, t8479: F, t3947: F, t687: F, t217: F, t219: F, t1024: F, t633: F, t3952: F, t654: F, t957: F, t682: F, t696: F, t978: F, t1066: F, t1108: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8482 = 48.245938496077606 * t1041 * t8479 * t1043;
    let t8483 = t3947 * t687;
    let t8485 = 1.0 / t217;
    let t8499 = 1.0 / t219;
    let t8519 = 6.0 * t1024 * t8479 * t633;
    let t8520 = t3952 * t654;
    let t8522 = t957 * t957;
    let t8526 = 3.5089341735807875 * t696 * t978 * t8522 * t682;
    let t8527 = t1108 * t1066;
    (t8482, t8483, t8485, t8499, t8519, t8520, t8522, t8526, t8527)
}
