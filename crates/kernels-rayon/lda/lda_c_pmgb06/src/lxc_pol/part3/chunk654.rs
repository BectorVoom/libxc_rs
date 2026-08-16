//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 654/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk654(t1108: f64, t687: f64, t110: f64, t980: f64, t1121: f64, t410: f64, t698: f64, t959: f64, t968: f64, t30: f64, t653: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3955 = t1108 * t687;
    let t3956 = 96.0_f64 * t3955;
    let t3957 = t110 * t980;
    let t3959 = 0.03253074390090522_f64 * t1121 * t3957;
    let t3960 = t410 * t698;
    let t3962 = 0.02168716260060348_f64 * t1121 * t3960;
    let t3963 = t110 * t959;
    let t3965 = 0.01626537195045261_f64 * t1121 * t3963;
    let t3966 = t110 * t968;
    let t3968 = 0.4815973313767657_f64 * t1121 * t3966;
    let t3969 = t653 * t30;
    (t3955, t3956, t3957, t3959, t3960, t3962, t3963, t3965, t3966, t3968, t3969)
}
