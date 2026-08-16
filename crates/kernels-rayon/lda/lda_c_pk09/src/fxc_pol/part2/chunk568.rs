//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 568/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk568(t3330: f64, t3332: f64, t3339: f64, t759: f64, t174: f64, t769: f64, t10: f64, t88: f64, t119: f64, t703: f64, t3172: f64, t3767: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3870 = 1.5625_f64 * t3330;
    let t3871 = 2.0833333333333335_f64 * t3332;
    let t3873 = 0.3472222222222222_f64 * t3339;
    let t3886 = t759 * t759;
    let t3888 = 1.0_f64 / t3886 / t174;
    let t3890 = t769 * t769;
    let t3891 = 1.0_f64 / t3890;
    let t3893 = t3891 * t88 * t10;
    let t3897 = t703 * t119;
    let t3906 = t3767 * t3172;
    (t3870, t3871, t3873, t3888, t3893, t3897, t3906)
}
