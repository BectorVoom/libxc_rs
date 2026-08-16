//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1119/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1119(t1966: f64, t2064: f64, t439: f64, t6554: f64, t20420: f64, t20423: f64, t20428: f64, t20431: f64, t20435: f64, t20436: f64, t20438: f64, t20440: f64, t20442: f64, t20445: f64) -> (f64, f64) {
    let t20449 = t439 * t1966 * t6554 * t2064 / 5.0_f64;
    let t20450 = -t20420 - t20423 + t20428 - t20431 + t20435 - t20436 - t20438 - t20440 + t20442 + t20445 + t20449;
    (t20449, t20450)
}
