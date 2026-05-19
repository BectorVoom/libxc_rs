//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 921/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk921<F: Float>(t10524: F, t115: F, t247: F, t413: F, t113: F, t642: F, t8131: F, t8193: F, t342: F, t4044: F, t6007: F, t3650: F, t4359: F) -> (F, F, F, F, F, F, F) {
    let t10525 = t10524 / F::new(2.0);
    let t10528 = F::cast_from(0.007532237109403992_f64) * t413 * t247 * t115;
    let t10531 = F::cast_from(0.015064474218807983_f64) * t113 * t642 * t115;
    let t10532 = F::new(96.0) * t8131;
    let t10533 = F::new(60.0) * t8193;
    let t10541 = t6007 * t4044 * t342;
    let t10544 = t4359 * t3650;
    (t10525, t10528, t10531, t10532, t10533, t10541, t10544)
}
