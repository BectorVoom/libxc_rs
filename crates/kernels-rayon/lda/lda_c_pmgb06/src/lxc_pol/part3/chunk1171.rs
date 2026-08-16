//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1171/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1171(t132: f64, t137: f64, t13979: f64, t477: f64, t1423: f64, t5350: f64, t12389: f64, t1897: f64, t439: f64, t1385: f64, t3010: f64, t5271: f64) -> (f64, f64, f64, f64) {
    let t13983 = t132 * t137 * t13979 * t477 / 10.0_f64;
    let t13984 = t1423 * t5350;
    let t13985 = 4.0_f64 / 15.0_f64 * t13984;
    let t13988 = 8.0_f64 / 15.0_f64 * t439 * t1897 * t12389;
    let t13992 = 2.0_f64 / 15.0_f64 * t439 * t1385 * t5271 * t3010;
    (t13983, t13985, t13988, t13992)
}
