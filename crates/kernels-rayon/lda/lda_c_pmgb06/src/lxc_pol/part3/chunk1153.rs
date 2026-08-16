//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1153/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1153(t1423: f64, t5261: f64, t5257: f64, t12343: f64, t1897: f64, t2010: f64, t5242: f64, t5245: f64, t12339: f64, t1901: f64, t439: f64, t5273: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13761 = t1423 * t5261;
    let t13762 = 16.0_f64 / 81.0_f64 * t13761;
    let t13763 = t1423 * t5257;
    let t13764 = 2.0_f64 / 27.0_f64 * t13763;
    let t13767 = 4.0_f64 / 5.0_f64 * t2010 * t1897 * t12343;
    let t13768 = t1423 * t5242;
    let t13769 = 4.0_f64 / 45.0_f64 * t13768;
    let t13770 = t1423 * t5245;
    let t13771 = 4.0_f64 / 9.0_f64 * t13770;
    let t13774 = 4.0_f64 / 3.0_f64 * t439 * t1901 * t12339;
    let t13775 = t1423 * t5273;
    (t13762, t13764, t13767, t13769, t13771, t13774, t13775)
}
