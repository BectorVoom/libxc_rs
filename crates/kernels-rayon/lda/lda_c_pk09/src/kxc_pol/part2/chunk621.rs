//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 621/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk621(t5012: f64, t5117: f64, t1285: f64, t4998: f64, t1329: f64, t1468: f64, t1387: f64, t1472: f64, t5039: f64, t5045: f64, t5068: f64, t1413: f64, t1416: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5119 = 38.978347549160304_f64 * t5117 * t5012;
    let t5121 = 12.992782516386768_f64 * t1285 * t4998;
    let t5122 = t1329 * t1468;
    let t5123 = t5122 * t1387;
    let t5124 = t5123 * t1472;
    let t5126 = 0.9421211958699838_f64 * t5039;
    let t5128 = 0.6280807972466558_f64 * t5045;
    let t5134 = 0.20936026574888528_f64 * t5068;
    let t5139 = t1413 * t1416;
    (t5119, t5121, t5123, t5124, t5126, t5128, t5134, t5139)
}
