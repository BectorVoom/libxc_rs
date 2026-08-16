//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 626/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk626(t3603: f64, t3559: f64, t38: f64, t56: f64, t370: f64, t3588: f64, t19: f64, t369: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3604 = 1.9486833333333333_f64 * t3603;
    let t3607 = 2.923025_f64 * t38 * t56 * t3559;
    let t3608 = t370 * t3559;
    let t3611 = t370 * t3588;
    let t3613 = 17.53815_f64 * t38 * t3611;
    let t3615 = 1.0_f64 / t369 / t19;
    (t3604, t3607, t3608, t3611, t3613, t3615)
}
