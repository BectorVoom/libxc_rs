//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 632/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk632(t1601: f64, t851: f64, t1531: f64, t465: f64, t2089: f64, t489: f64, t161: f64, t2100: f64, t1600: f64, t842: f64, t2018: f64, t486: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5090 = t1601 * t851;
    let t5094 = t465 * t1531;
    let t5102 = t489 * t2089;
    let t5104 = 2.0_f64 / 45.0_f64 * t161 * t5102;
    let t5105 = t489 * t2100;
    let t5107 = 2.0_f64 / 45.0_f64 * t161 * t5105;
    let t5108 = t842 * t1600;
    let t5114 = 2.0_f64 / 45.0_f64 * t486 * t2018;
    (t5090, t5094, t5102, t5104, t5105, t5107, t5108, t5114)
}
