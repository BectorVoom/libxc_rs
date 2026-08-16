//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1021/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1021(t146: f64, t164: f64, t9712: f64, t9501: f64, t132: f64, t1547: f64, t1630: f64, t1980: f64, t604: f64, t223: f64, t5210: f64, t1710: f64, t1727: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9981 = 0.10864197530864197_f64 * t146 * t9712 * t164;
    let t9986 = 0.3732469135802469_f64 * t9501;
    let t10046 = t132 * t1547 * t1630;
    let t10079 = t604 * t1980;
    let t10082 = 56.0_f64 / 1215.0_f64 * t223 * t5210;
    let t10085 = t1727 * t1710;
    (t9981, t9986, t10046, t10079, t10082, t10085)
}
