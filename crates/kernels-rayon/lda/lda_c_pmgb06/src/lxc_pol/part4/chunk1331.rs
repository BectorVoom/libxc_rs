//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1331/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1331(t13314: f64, t432: f64, t6572: f64, t132: f64, t137: f64, t2064: f64, t4815: f64, t4979: f64, t802: f64, t1887: f64, t2015: f64, t4966: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17497 = 16.0_f64 / 135.0_f64 * t13314;
    let t17499 = 2.0_f64 / 15.0_f64 * t432 * t6572;
    let t17503 = 2.0_f64 / 15.0_f64 * t132 * t137 * t4815 * t2064;
    let t17505 = t802 * t4979 / 15.0_f64;
    let t17506 = t1887 * t2015;
    let t17507 = 4.0_f64 / 45.0_f64 * t17506;
    let t17509 = t802 * t4966 / 15.0_f64;
    (t17497, t17499, t17503, t17505, t17507, t17509)
}
