//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1109/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1109(t16697: f64, t16699: f64, t16701: f64, t3279: f64, t439: f64, t7645: f64, t2493: f64, t5187: f64, t2002: f64, t6297: f64, t1420: f64, t7651: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20323 = 4.0_f64 / 45.0_f64 * t16697;
    let t20324 = 4.0_f64 / 45.0_f64 * t16699;
    let t20325 = 8.0_f64 / 45.0_f64 * t16701;
    let t20328 = 2.0_f64 / 9.0_f64 * t439 * t3279 * t7645;
    let t20330 = 2.0_f64 / 15.0_f64 * t5187 * t2493;
    let t20332 = 2.0_f64 / 15.0_f64 * t2002 * t6297;
    let t20334 = 2.0_f64 / 15.0_f64 * t1420 * t7651;
    (t20323, t20324, t20325, t20328, t20330, t20332, t20334)
}
