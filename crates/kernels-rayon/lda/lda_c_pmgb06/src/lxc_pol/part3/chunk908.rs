//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 908/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk908(t9501: f64, t3347: f64, t405: f64, t3341: f64, t132: f64, t3046: f64, t435: f64, t1547: f64, t1630: f64, t1980: f64, t604: f64, t223: f64, t5210: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9986 = 0.3732469135802469_f64 * t9501;
    let t9987 = t405 * t3347;
    let t10006 = t405 * t3341;
    let t10040 = t132 * t435 * t3046;
    let t10046 = t132 * t1547 * t1630;
    let t10079 = t604 * t1980;
    let t10082 = 56.0_f64 / 1215.0_f64 * t223 * t5210;
    (t9986, t9987, t10006, t10040, t10046, t10079, t10082)
}
