//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1137/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1137(t14909: f64, t14933: f64, t257: f64, t1122: f64, t2395: f64, t30: f64, t6037: f64, t959: f64, t968: f64, t273: f64, t6067: f64, t698: f64) -> (f64, f64, f64, f64, f64) {
    let t14935 = (t14909 + t14933) * t257;
    let t14939 = t2395 * t30 * t1122;
    let t14942 = t6037 * t959;
    let t14944 = t6037 * t968;
    let t14947 = t6067 * t273 * t698;
    (t14935, t14939, t14942, t14944, t14947)
}
