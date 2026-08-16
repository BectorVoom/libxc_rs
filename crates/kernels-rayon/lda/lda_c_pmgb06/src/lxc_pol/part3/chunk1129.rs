//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1129/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1129(t13378: f64, t13416: f64, t161: f64, t166: f64, t176: f64, t486: f64, t5417: f64, t3146: f64, t844: f64, t1499: f64, t1837: f64, t1417: f64, t5305: f64) -> (f64, f64, f64, f64, f64) {
    let t13421 = t161 * t166 * (t13378 + t13416) * t176 / 30.0_f64;
    let t13423 = t486 * t5417 / 10.0_f64;
    let t13425 = t3146 * t844 / 30.0_f64;
    let t13427 = t1499 * t1837 / 10.0_f64;
    let t13429 = 2.0_f64 / 15.0_f64 * t5305 * t1417;
    (t13421, t13423, t13425, t13427, t13429)
}
