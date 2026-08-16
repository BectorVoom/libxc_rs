//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1170/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1170(t1420: f64, t5345: f64, t486: f64, t5102: f64, t1499: f64, t2018: f64, t132: f64, t443: f64, t459: f64, t4828: f64, t464: f64, t4680: f64) -> (f64, f64, f64, f64, f64) {
    let t13970 = 2.0_f64 / 15.0_f64 * t1420 * t5345;
    let t13971 = t486 * t5102;
    let t13972 = 2.0_f64 / 15.0_f64 * t13971;
    let t13973 = t1499 * t2018;
    let t13974 = t13973 / 15.0_f64;
    let t13978 = 2.0_f64 / 15.0_f64 * t132 * t4828 * t459 * t443;
    let t13979 = t4680 * t464;
    (t13970, t13972, t13974, t13978, t13979)
}
