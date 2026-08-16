//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1116/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1116(t10087: f64, t10089: f64, t1467: f64, t5305: f64, t1972: f64, t3195: f64, t3235: f64, t3239: f64, t1963: f64, t3177: f64, t1420: f64, t4615: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13257 = t10087 / 45.0_f64;
    let t13258 = 2.0_f64 / 45.0_f64 * t10089;
    let t13260 = t5305 * t1467 / 9.0_f64;
    let t13262 = t1972 * t3195 / 15.0_f64;
    let t13264 = t1972 * t3235 / 15.0_f64;
    let t13266 = t1972 * t3239 / 9.0_f64;
    let t13268 = t3177 * t1963 / 15.0_f64;
    let t13270 = t1420 * t4615 / 15.0_f64;
    (t13257, t13258, t13260, t13262, t13264, t13266, t13268, t13270)
}
