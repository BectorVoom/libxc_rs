//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 598/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk598(t396: f64, t4767: f64, t1494: f64, t1653: f64, t1493: f64, t296: f64, t343: f64, t366: f64, t1191: f64, t1142: f64, t13: f64, t1147: f64, t229: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4769 = 0.9840332968370255_f64 * t396 * t4767;
    let t4770 = t1494 * t1653;
    let t4774 = 1.0_f64 / t1493 / t296;
    let t4775 = t4774 * t343;
    let t4782 = 2.1943705410881575_f64 * t366 * t4767;
    let t4785 = t1191 * t1191;
    let t4787 = t13 * t1142;
    let t4789 = t229 * t1147;
    (t4769, t4770, t4775, t4782, t4785, t4787, t4789)
}
