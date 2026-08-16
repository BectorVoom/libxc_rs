//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 993/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk993(t2002: f64, t3191: f64, t9291: f64, t9293: f64, t9295: f64, t9297: f64, t1179: f64, t4068: f64, t871: f64, t11790: f64, t11793: f64, t11795: f64, t11796: f64, t11799: f64, t11802: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11804 = 2.0_f64 / 9.0_f64 * t2002 * t3191;
    let t11805 = 2.0_f64 / 45.0_f64 * t9291;
    let t11806 = 4.0_f64 / 45.0_f64 * t9293;
    let t11807 = 2.0_f64 / 27.0_f64 * t9295;
    let t11808 = 2.0_f64 / 27.0_f64 * t9297;
    let t11810 = t871 * t1179 * t4068;
    let t11812 = -t11790 - t11793 - t11795 + 0.09973633333333333_f64 * t11796 + t11799 + t11802 - t11804 - t11805 - t11806 + t11807 - t11808 + 0.001515438175925926_f64 * t11810;
    (t11804, t11805, t11806, t11807, t11808, t11812)
}
