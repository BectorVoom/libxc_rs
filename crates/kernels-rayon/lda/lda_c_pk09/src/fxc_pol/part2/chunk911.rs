//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 911/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk911(t2964: f64, t4861: f64, t9673: f64, t9674: f64, t1180: f64, t1195: f64, t253: f64, t4803: f64, t4806: f64, t4807: f64, t4809: f64, t4817: f64, t4819: f64, t4824: f64, t4833: f64, t9643: f64, t9646: f64, t9653: f64, t9657: f64, t9660: f64, t9664: f64, t9666: f64, t9670: f64) -> (f64, f64) {
    let t9675 = 15.13129101521689_f64 * t2964;
    let t9676 = t9673 - t9674 - t9675 + t4861;
    let t9677 = t1180 * t9676;
    let t9680 = t4803 - t4806 + 1.28_f64 * t4807 - 1.28_f64 * t4809 + t4817 - 1.28_f64 * t4819 + 1.28_f64 * t4824 - t4833 + 1.28_f64 * t9643 - 1.28_f64 * t9646 + 1.28_f64 * t253 * t9653 - 1.28_f64 * t253 * t9657 - 1.28_f64 * t9660 + 1.28_f64 * t9664 - 1.28_f64 * t253 * t9666 + 2.56_f64 * t1195 * t9670 - 1.28_f64 * t253 * t9677;
    (t9675, t9680)
}
