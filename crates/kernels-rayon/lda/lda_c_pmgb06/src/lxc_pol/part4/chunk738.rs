//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 738/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk738(t1966: f64, t4766: f64, t439: f64, t1417: f64, t1972: f64, t1559: f64, t1962: f64, t1560: f64, t2002: f64, t3213: f64, t806: f64, t1872: f64, t441: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4767 = t1966 * t4766;
    let t4769 = t439 * t4767 / 5.0_f64;
    let t4771 = 2.0_f64 / 45.0_f64 * t1972 * t1417;
    let t4772 = t1962 * t1559;
    let t4774 = 2.0_f64 / 45.0_f64 * t439 * t4772;
    let t4776 = 2.0_f64 / 45.0_f64 * t2002 * t1560;
    let t4777 = t3213 * t806;
    let t4778 = 2.0_f64 / 405.0_f64 * t4777;
    let t4779 = t441 * t1872;
    (t4767, t4769, t4771, t4772, t4774, t4776, t4777, t4778, t4779)
}
