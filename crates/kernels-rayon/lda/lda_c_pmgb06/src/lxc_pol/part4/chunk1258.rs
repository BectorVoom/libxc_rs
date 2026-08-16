//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1258/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1258(t5120: f64, t802: f64, t12981: f64, t6633: f64, t1594: f64, t3032: f64, t443: f64, t5077: f64, t6637: f64, t13007: f64, t6562: f64, t1602: f64, t3458: f64, t497: f64, t5068: f64, t6560: f64) -> (f64, f64, f64, f64, f64) {
    let t16541 = 2.0_f64 / 15.0_f64 * t802 * t5120;
    let t16542 = t12981 * t6633;
    let t16543 = 8.0_f64 / 81.0_f64 * t16542;
    let t16548 = 4.0_f64 / 15.0_f64 * t5077 * t3032 * t443 * t6637 * t1594;
    let t16549 = t13007 * t6562;
    let t16550 = 16.0_f64 / 135.0_f64 * t16549;
    let t16555 = 4.0_f64 / 15.0_f64 * t5068 * t3458 * t497 * t6560 * t1602;
    (t16541, t16543, t16548, t16550, t16555)
}
