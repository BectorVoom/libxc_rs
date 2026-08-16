//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 632/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk632(t232: f64, t3674: f64, t1043: f64, t3667: f64, t28: f64, t3500: f64, t247: f64, t740: f64, t934: f64, t940: f64, t2781: f64, t623: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3675 = t232 * t3674;
    let t3676 = t3667 * t1043;
    let t3678 = 96.49187699215521_f64 * t3675 * t3676;
    let t3679 = t3500 * t28;
    let t3680 = t3679 * t247;
    let t3682 = t934 * t740;
    let t3683 = t940 * t3682;
    let t3685 = t623 * t2781;
    (t3675, t3676, t3678, t3679, t3680, t3682, t3683, t3685)
}
