//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 845/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk845(t1763: f64, t1773: f64, t4294: f64, t707: f64, t100: f64, t1099: f64, t1193: f64, t4299: f64, t83: f64, t1530: f64, t9: f64, t1: f64, t642: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8097 = 0.31931290694012293_f64 * t1773 * t1763;
    let t8099 = 0.07982822673503073_f64 * t707 * t4294;
    let t8101 = 1.0_f64 / t100 / t1099;
    let t8105 = 6.701521338562081e-05_f64 * t8101 * t83 * t1193 * t4299;
    let t8119 = 1.0_f64 / t9 / t1530;
    let t8131 = t1 * t642;
    (t8097, t8099, t8101, t8105, t8119, t8131)
}
