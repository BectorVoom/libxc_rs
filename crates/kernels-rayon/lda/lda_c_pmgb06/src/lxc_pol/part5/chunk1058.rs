//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1058/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1058(t16051: f64, t16053: f64, t16055: f64, t16057: f64, t16089: f64, t1969: f64, t6127: f64, t6584: f64, t802: f64, t1887: f64, t2650: f64, t132: f64, t137: f64, t1395: f64, t7801: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19679 = 4.0_f64 / 45.0_f64 * t16051;
    let t19680 = 4.0_f64 / 15.0_f64 * t16053;
    let t19681 = 4.0_f64 / 9.0_f64 * t16055;
    let t19682 = 16.0_f64 / 45.0_f64 * t16057;
    let t19683 = 2.0_f64 / 15.0_f64 * t16089;
    let t19685 = t6127 * t1969 / 5.0_f64;
    let t19687 = t802 * t6584 / 10.0_f64;
    let t19689 = t1887 * t2650 / 10.0_f64;
    let t19693 = t132 * t137 * t1395 * t7801 / 30.0_f64;
    (t19679, t19680, t19681, t19682, t19683, t19685, t19687, t19689, t19693)
}
