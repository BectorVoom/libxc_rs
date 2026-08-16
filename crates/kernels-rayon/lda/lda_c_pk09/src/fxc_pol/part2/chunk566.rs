//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 566/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk566(t3339: f64, t3330: f64, t3444: f64, t3453: f64, t2971: f64, t983: f64, t2974: f64, t141: f64, t154: f64, t119: f64, t975: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3794 = 0.010056629776875343_f64 * t3339;
    let t3803 = 0.04525483399593904_f64 * t3330;
    let t3810 = 0.6806222787477182_f64 * t3444;
    let t3812 = 1.8149927433272484_f64 * t3453;
    let t3820 = t983 * t2971;
    let t3821 = t3820 * t2974;
    let t3823 = t141 * t2971;
    let t3826 = t154 * t2971;
    let t3829 = t975 * t119;
    let t3835 = t973 * t973;
    (t3794, t3803, t3810, t3812, t3820, t3821, t3823, t3826, t3829, t3835)
}
