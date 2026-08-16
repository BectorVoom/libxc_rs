//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1066/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1066(t4713: f64, t607: f64, t1710: f64, t1959: f64, t432: f64, t4979: f64, t9616: f64, t9619: f64, t12648: f64, t12650: f64, t12653: f64, t12654: f64, t12655: f64, t12656: f64, t12657: f64) -> (f64, f64, f64, f64) {
    let t12659 = t4713 * t607;
    let t12661 = t1959 * t1710;
    let t12662 = 2.0_f64 / 45.0_f64 * t12661;
    let t12664 = t432 * t4979 / 10.0_f64;
    let t12665 = t9616 / 15.0_f64;
    let t12666 = 2.0_f64 / 45.0_f64 * t9619;
    let t12667 = -t12648 - t12650 - t12653 - t12654 + t12655 + t12656 - 8.0_f64 / 405.0_f64 * t12657 - 2.0_f64 / 15.0_f64 * t12659 + t12662 - t12664 - t12665 - t12666;
    (t12664, t12665, t12666, t12667)
}
