//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 949/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk949(t4524: f64, t643: f64, t638: f64, t3957: f64, t4549: f64, t3960: f64, t3966: f64, t1122: f64, t2142: f64, t30: f64, t3963: f64, t8685: f64, t8692: f64, t8693: f64, t8723: f64, t8724: f64, t8727: f64, t8729: f64, t8733: f64, t8737: f64) -> f64 {
    let t11110 = t643 * t4524;
    let t11112 = t638 * t4524;
    let t11113 = 12.0_f64 * t11112;
    let t11115 = t4549 * t3957;
    let t11117 = t4549 * t3960;
    let t11119 = t4549 * t3966;
    let t11122 = t2142 * t30 * t1122;
    let t11123 = 0.03253074390090522_f64 * t11122;
    let t11124 = t4549 * t3963;
    let t11126 = -3076.205657464922_f64 * t8685 + t8692 - 1.7544670867903938_f64 * t8693 - t8723 + 311.68546390226635_f64 * t8724 - 12.0_f64 * t11110 + t11113 + t8727 - 4.0_f64 * t8729 + t8733 - 0.03253074390090522_f64 * t11115 - 0.02168716260060348_f64 * t11117 + 0.4815973313767657_f64 * t11119 + t11123 + 0.01626537195045261_f64 * t11124 - t8737;
    t11126
}
