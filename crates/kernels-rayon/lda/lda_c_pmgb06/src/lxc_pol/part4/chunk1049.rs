//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1049/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1049(t4524: f64, t643: f64, t638: f64, t3957: f64, t4549: f64, t3960: f64, t3966: f64, t1122: f64, t2142: f64, t30: f64, t3963: f64, t1105: f64, t2160: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11110 = t643 * t4524;
    let t11112 = t638 * t4524;
    let t11115 = t4549 * t3957;
    let t11117 = t4549 * t3960;
    let t11119 = t4549 * t3966;
    let t11122 = t2142 * t30 * t1122;
    let t11124 = t4549 * t3963;
    let t11135 = t1105 * t2160;
    (t11110, t11112, t11115, t11117, t11119, t11122, t11124, t11135)
}
