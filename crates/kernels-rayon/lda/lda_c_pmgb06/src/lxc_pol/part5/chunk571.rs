//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 571/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk571(t232: f64, t3665: f64, t1025: f64, t632: f64, t1042: f64, t241: f64, t1039: f64, t238: f64, t1043: f64, t28: f64, t3500: f64, t247: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3666 = t232 * t3665;
    let t3667 = t1025 * t632;
    let t3669 = 1.0_f64 / t1042 / t241;
    let t3670 = t3667 * t3669;
    let t3672 = 517.260129192734_f64 * t3666 * t3670;
    let t3674 = 1.0_f64 / t1039 / t238;
    let t3675 = t232 * t3674;
    let t3676 = t3667 * t1043;
    let t3678 = 96.49187699215521_f64 * t3675 * t3676;
    let t3679 = t3500 * t28;
    let t3680 = t3679 * t247;
    (t3666, t3667, t3669, t3670, t3672, t3674, t3675, t3676, t3678, t3679, t3680)
}
