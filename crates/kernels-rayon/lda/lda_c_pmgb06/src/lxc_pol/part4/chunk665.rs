//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 665/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk665(t1112: f64, t974: f64, t1039: f64, t620: f64, t232: f64, t1025: f64, t632: f64, t1042: f64, t241: f64, t238: f64, t1043: f64, t28: f64, t3500: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3662 = t974 * t1112;
    let t3665 = 1.0_f64 / t1039 / t620;
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
    (t3662, t3665, t3666, t3667, t3669, t3670, t3672, t3674, t3675, t3676, t3678, t3679)
}
