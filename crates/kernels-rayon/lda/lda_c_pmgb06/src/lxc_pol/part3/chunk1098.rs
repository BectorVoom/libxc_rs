//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1098/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1098(t13079: f64, t9890: f64, t9892: f64, t13056: f64, t13060: f64, t13063: f64, t13067: f64, t13071: f64, t13074: f64, t13075: f64, t13076: f64, t9895: f64) -> (f64, f64, f64, f64, f64) {
    let t13080 = 8.0_f64 / 45.0_f64 * t13079;
    let t13081 = 4.0_f64 / 135.0_f64 * t9890;
    let t13082 = 2.0_f64 / 45.0_f64 * t9892;
    let t13083 = -t13056 - t13060 - t13063 + t13067 + t13071 + t13074 - t13075 - t13076 - t13080 - t13081 + t13082;
    let t13084 = 2.0_f64 / 45.0_f64 * t9895;
    (t13080, t13081, t13082, t13083, t13084)
}
