//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1164/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1164(t1420: f64, t7542: f64, t439: f64, t5225: f64, t7493: f64, t1897: f64, t19778: f64, t443: f64, t7801: f64, t1385: f64, t332: f64, t5482: f64, t6774: f64) -> (f64, f64, f64, f64, f64) {
    let t20981 = 2.0_f64 / 15.0_f64 * t1420 * t7542;
    let t20984 = 2.0_f64 / 15.0_f64 * t439 * t5225 * t7493;
    let t20987 = 2.0_f64 / 15.0_f64 * t439 * t1897 * t19778;
    let t20988 = t7801 * t443;
    let t20992 = t439 * t1385 * t20988 * t332 / 45.0_f64;
    let t20995 = t439 * t5482 * t6774 / 15.0_f64;
    (t20981, t20984, t20987, t20992, t20995)
}
