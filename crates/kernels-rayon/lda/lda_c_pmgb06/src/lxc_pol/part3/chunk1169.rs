//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1169/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1169(t10439: f64, t3033: f64, t439: f64, t809: f64, t2002: f64, t2957: f64, t2948: f64, t5344: f64, t1069: f64, t1385: f64, t1531: f64, t2064: f64) -> (f64, f64, f64, f64) {
    let t13958 = 2.0_f64 / 15.0_f64 * t439 * t10439 * t809 * t3033;
    let t13960 = t2002 * t2957 / 15.0_f64;
    let t13963 = 2.0_f64 / 15.0_f64 * t439 * t2948 * t5344;
    let t13968 = 2.0_f64 / 15.0_f64 * t439 * t1385 * t2064 * t1531 * t1069;
    (t13958, t13960, t13963, t13968)
}
