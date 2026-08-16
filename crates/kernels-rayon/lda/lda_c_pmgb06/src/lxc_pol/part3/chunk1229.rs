//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1229/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1229(t5887: f64, t707: f64, t5891: f64, t5895: f64, t1770: f64, t419: f64, t4238: f64, t794: f64, t4044: f64, t6007: f64, t769: f64, t1289: f64, t4232: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14567 = t707 * t5887;
    let t14569 = t707 * t5891;
    let t14570 = 0.11974234010254609_f64 * t14569;
    let t14571 = t707 * t5895;
    let t14575 = t4238 * t794 * t419 * t1770;
    let t14587 = t6007 * t769 * t4044;
    let t14593 = t4232 * t769 * t1289;
    (t14567, t14570, t14571, t14575, t14587, t14593)
}
