//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1361/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1361(t14017: f64, t14019: f64, t1916: f64, t5194: f64, t1972: f64, t5333: f64, t1920: f64, t1594: f64, t2570: f64, t439: f64, t9084: f64, t15349: f64, t1897: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17884 = 4.0_f64 / 45.0_f64 * t14017;
    let t17885 = 4.0_f64 / 45.0_f64 * t14019;
    let t17886 = t5194 * t1916;
    let t17887 = 16.0_f64 / 135.0_f64 * t17886;
    let t17889 = 4.0_f64 / 15.0_f64 * t1972 * t5333;
    let t17890 = t5194 * t1920;
    let t17891 = 8.0_f64 / 81.0_f64 * t17890;
    let t17895 = 2.0_f64 / 27.0_f64 * t439 * t9084 * t2570 * t1594;
    let t17898 = 2.0_f64 / 15.0_f64 * t439 * t1897 * t15349;
    (t17884, t17885, t17887, t17889, t17891, t17895, t17898)
}
