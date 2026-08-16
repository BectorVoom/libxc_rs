//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 928/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk928(t2617: f64, t405: f64, t2620: f64, t2614: f64, t525: f64, t6827: f64, t1576: f64, t6503: f64, t3358: f64, t6508: f64, t6512: f64, t6402: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6873 = t405 * t2617;
    let t6875 = t405 * t2620;
    let t6877 = t405 * t2614;
    let t6879 = t525 * t6827;
    let t6882 = t1576 * t6503;
    let t6885 = t3358 * t6508;
    let t6888 = t1576 * t6512;
    let t6891 = t525 * t6402;
    (t6873, t6875, t6877, t6879, t6882, t6885, t6888, t6891)
}
