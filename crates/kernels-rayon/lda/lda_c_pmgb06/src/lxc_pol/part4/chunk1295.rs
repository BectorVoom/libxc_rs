//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1295/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1295(t1447: f64, t6287: f64, t6528: f64, t2002: f64, t5171: f64, t16979: f64, t16980: f64, t16981: f64, t16983: f64, t16985: f64, t16988: f64, t16991: f64, t16993: f64, t16996: f64, t16998: f64, t17000: f64, t17003: f64) -> (f64, f64, f64, f64) {
    let t17004 = t1447 * t6287;
    let t17005 = 4.0_f64 / 15.0_f64 * t17004;
    let t17006 = t1447 * t6528;
    let t17007 = 8.0_f64 / 45.0_f64 * t17006;
    let t17009 = 2.0_f64 / 45.0_f64 * t2002 * t5171;
    let t17010 = t16979 + t16980 + t16981 + t16983 + t16985 + t16988 + t16991 + t16993 - t16996 - t16998 + t17000 + t17003 - t17005 + t17007 + t17009;
    (t17005, t17007, t17009, t17010)
}
