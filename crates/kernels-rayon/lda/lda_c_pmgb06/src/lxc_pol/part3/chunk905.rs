//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 905/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk905(t3453: f64, t486: f64, t161: f64, t3004: f64, t530: f64, t3450: f64, t1554: f64, t1640: f64, t1603: f64, t3457: f64, t496: f64, t1382: f64, t3223: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9887 = t486 * t3453;
    let t9890 = t161 * t3004 * t530;
    let t9892 = t486 * t3450;
    let t9895 = t161 * t1554 * t1640;
    let t9898 = t161 * t1554 * t1603;
    let t9908 = t496 * t3457;
    let t9921 = t3223 * t1382;
    (t9887, t9890, t9892, t9895, t9898, t9908, t9921)
}
