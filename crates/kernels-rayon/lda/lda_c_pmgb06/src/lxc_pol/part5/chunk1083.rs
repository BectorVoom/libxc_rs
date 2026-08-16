//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1083/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1083(t2991: f64, t493: f64, t529: f64, t7598: f64, t19371: f64, t5470: f64, t1919: f64, t19389: f64, t1981: f64, t1385: f64, t15935: f64, t439: f64, t760: f64) -> (f64, f64, f64, f64) {
    let t20025 = 2.0_f64 / 9.0_f64 * t493 * t2991 * t7598 * t529;
    let t20028 = 32.0_f64 / 27.0_f64 * t493 * t5470 * t19371;
    let t20031 = 4.0_f64 / 3.0_f64 * t1981 * t1919 * t19389;
    let t20035 = t439 * t1385 * t15935 * t760 / 15.0_f64;
    (t20025, t20028, t20031, t20035)
}
