//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1118/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1118(t1397: f64, t1887: f64, t3060: f64, t802: f64, t161: f64, t4839: f64, t497: f64, t512: f64, t10099: f64, t10101: f64, t10103: f64, t10105: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13277 = t1887 * t1397 / 5.0_f64;
    let t13279 = t802 * t3060 / 10.0_f64;
    let t13283 = 2.0_f64 / 15.0_f64 * t161 * t4839 * t512 * t497;
    let t13284 = 2.0_f64 / 81.0_f64 * t10099;
    let t13285 = 4.0_f64 / 27.0_f64 * t10101;
    let t13286 = 2.0_f64 / 45.0_f64 * t10103;
    let t13287 = 2.0_f64 / 27.0_f64 * t10105;
    (t13277, t13279, t13283, t13284, t13285, t13286, t13287)
}
