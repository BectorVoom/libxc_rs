//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1136/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1136(t1318: f64, t2526: f64, t5269: f64, t593: f64, t811: f64, t16050: f64, t16053: f64, t16058: f64, t16065: f64, t568: f64, t7676: f64, t2023: f64, t6205: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21001 = 8.0_f64 / 5.0_f64 * t1318 * t5269 * t2526 * t811 * t593;
    let t21002 = 32.0_f64 / 45.0_f64 * t16050;
    let t21003 = 32.0_f64 / 45.0_f64 * t16053;
    let t21004 = 16.0_f64 / 135.0_f64 * t16058;
    let t21005 = 8.0_f64 / 45.0_f64 * t16065;
    let t21007 = t7676 * t568;
    let t21008 = 4.0_f64 / 45.0_f64 * t21007;
    let t21012 = 4.0_f64 / 15.0_f64 * t6205 * t2023;
    (t21001, t21002, t21003, t21004, t21005, t21008, t21012)
}
