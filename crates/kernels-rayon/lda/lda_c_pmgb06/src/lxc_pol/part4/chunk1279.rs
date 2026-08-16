//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1279/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1279(t1074: f64, t6637: f64, t5077: f64, t5094: f64, t1069: f64, t13000: f64, t5083: f64, t1: f64, t822: f64, t332: f64, t13043: f64, t13047: f64, t6646: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16821 = t6637 * t1074;
    let t16824 = 4.0_f64 / 45.0_f64 * t5077 * t5094 * t16821;
    let t16825 = t6637 * t1069;
    let t16828 = 4.0_f64 / 9.0_f64 * t5083 * t13000 * t16825;
    let t16829 = t1 * t822;
    let t16830 = t16829 * t332;
    let t16833 = 16.0_f64 / 45.0_f64 * t13043 * t5094 * t16830;
    let t16835 = 4.0_f64 / 27.0_f64 * t13047 * t6646;
    (t16821, t16824, t16825, t16828, t16829, t16830, t16833, t16835)
}
