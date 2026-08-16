//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1022/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1022(t12154: f64, t12156: f64, t439: f64, t1: f64, t1069: f64, t3092: f64, t2010: f64, t5260: f64, t1074: f64, t4667: f64, t1897: f64, t1420: f64, t5245: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12159 = 88.0_f64 / 243.0_f64 * t439 * t12154 * t12156;
    let t12161 = t3092 * t1 * t1069;
    let t12164 = 16.0_f64 / 27.0_f64 * t2010 * t5260 * t12161;
    let t12165 = t4667 * t1074;
    let t12168 = 4.0_f64 / 15.0_f64 * t2010 * t1897 * t12165;
    let t12170 = 2.0_f64 / 3.0_f64 * t1420 * t5245;
    (t12159, t12161, t12164, t12165, t12168, t12170)
}
