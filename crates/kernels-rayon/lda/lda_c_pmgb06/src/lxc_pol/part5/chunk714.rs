//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 714/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk714(t439: f64, t6556: f64, t1601: f64, t497: f64, t764: f64, t851: f64, t529: f64, t5068: f64, t3156: f64, t3214: f64, t3224: f64, t5186: f64, t6526: f64, t6530: f64, t6532: f64, t6535: f64, t6538: f64, t6540: f64, t6543: f64, t6547: f64, t6549: f64, t6553: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6558 = t439 * t6556 / 15.0_f64;
    let t6559 = t1601 * t497;
    let t6560 = t764 * t851;
    let t6561 = t6560 * t529;
    let t6562 = t6559 * t6561;
    let t6564 = 4.0_f64 / 45.0_f64 * t5068 * t6562;
    let t6565 = t3156 / 135.0_f64;
    let t6566 = 2.0_f64 / 405.0_f64 * t3214;
    let t6567 = 2.0_f64 / 405.0_f64 * t3224;
    let t6568 = t6526 + t6530 + t6532 + t6535 - t6538 + t6540 + t6543 + t6547 + t6549 + t6553 + t6558 + t6564 - t6565 - t6566 - t6567 + t5186;
    (t6558, t6559, t6560, t6562, t6564, t6565, t6566, t6567, t6568)
}
