//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1344/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1344(t4817: f64, t802: f64, t165: f64, t1835: f64, t1994: f64, t493: f64, t13706: f64, t439: f64, t5202: f64, t6550: f64, t1423: f64, t6259: f64) -> (f64, f64, f64, f64, f64) {
    let t17657 = 2.0_f64 / 15.0_f64 * t802 * t4817;
    let t17661 = 4.0_f64 / 15.0_f64 * t493 * t165 * t1835 * t1994;
    let t17662 = 4.0_f64 / 135.0_f64 * t13706;
    let t17665 = 4.0_f64 / 15.0_f64 * t439 * t6550 * t5202;
    let t17666 = t1423 * t6259;
    (t17657, t17661, t17662, t17665, t17666)
}
