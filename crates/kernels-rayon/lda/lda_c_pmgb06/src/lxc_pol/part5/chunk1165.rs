//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1165/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1165(t10445: f64, t493: f64, t529: f64, t7594: f64, t2002: f64, t6416: f64, t6419: f64, t6465: f64, t6475: f64, t6275: f64, t6478: f64, t20981: f64, t20984: f64, t20987: f64, t20992: f64, t20995: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20999 = 8.0_f64 / 81.0_f64 * t493 * t10445 * t7594 * t529;
    let t21001 = 2.0_f64 / 15.0_f64 * t2002 * t6416;
    let t21003 = t2002 * t6419 / 9.0_f64;
    let t21005 = t2002 * t6465 / 9.0_f64;
    let t21007 = 8.0_f64 / 27.0_f64 * t2002 * t6475;
    let t21009 = 4.0_f64 / 9.0_f64 * t6275 * t6478;
    let t21010 = -t20981 - t20984 - t20987 - t20992 - t20995 - t20999 - t21001 + t21003 - t21005 + t21007 + t21009;
    (t20999, t21001, t21003, t21005, t21007, t21009, t21010)
}
