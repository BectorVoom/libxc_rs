//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1312/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1312(t13230: f64, t13232: f64, t13235: f64, t13237: f64, t13239: f64, t13241: f64, t17012: f64, t17013: f64, t17014: f64, t17015: f64, t17017: f64, t17018: f64, t17020: f64, t17252: f64, t17253: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17254 = 2.0_f64 / 45.0_f64 * t13230;
    let t17255 = 4.0_f64 / 45.0_f64 * t13232;
    let t17256 = 2.0_f64 / 45.0_f64 * t13235;
    let t17257 = 2.0_f64 / 45.0_f64 * t13237;
    let t17258 = 4.0_f64 / 45.0_f64 * t13239;
    let t17259 = 8.0_f64 / 135.0_f64 * t13241;
    let t17260 = t17012 + t17013 + t17014 + t17015 + t17017 + t17018 + t17020 - t17252 + t17253 + t17254 + t17255 + t17256 + t17257 + t17258 + t17259;
    (t17254, t17255, t17256, t17257, t17258, t17259, t17260)
}
