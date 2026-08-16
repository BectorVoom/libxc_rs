//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 703/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk703(t1672: f64, t1820: f64, t6319: f64, t6325: f64, t6464: f64, t538: f64, t6601: f64, t1146: f64, t132: f64, t142: f64, t550: f64, t2005: f64, t443: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6743 = t1820 * t1672;
    let t6747 = 11.879313099038017_f64 * t6319;
    let t6749 = 7.919542066025344_f64 * t6325;
    let t6755 = 2.6398473553417814_f64 * t6464;
    let t6764 = 0.9840332968370255_f64 * t538 * t6601;
    let t6769 = t142 * t1146 * t132;
    let t6771 = 3.948986526768806_f64 * t550 * t6769;
    let t6780 = 1.0_f64 / t2005 / t443;
    (t6743, t6747, t6749, t6755, t6764, t6771, t6780)
}
