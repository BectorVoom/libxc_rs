//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 936/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk936(t350: f64, t6973: f64, t2699: f64, t348: f64, t5980: f64, t64: f64, t35: f64, t110: f64, t2703: f64, t360: f64, t2707: f64, t2695: f64, t3615: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6974 = t6973 * t350;
    let t6975 = 0.9743416666666667_f64 * t6974;
    let t6976 = t348 * t2699;
    let t6977 = t6976 * t350;
    let t6978 = 0.48717083333333333_f64 * t6977;
    let t6979 = t64 * t5980;
    let t6980 = t35 * t6979;
    let t6983 = t110 * t2703;
    let t6984 = t360 * t6983;
    let t6986 = t110 * t2707;
    let t6987 = t360 * t6986;
    let t6989 = t3615 * t2695;
    (t6975, t6976, t6978, t6979, t6980, t6983, t6984, t6986, t6987, t6989)
}
