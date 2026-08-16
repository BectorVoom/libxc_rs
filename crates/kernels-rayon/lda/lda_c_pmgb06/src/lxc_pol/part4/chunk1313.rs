//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1313/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1313(t13243: f64, t13245: f64, t13249: f64, t13251: f64, t10087: f64, t10089: f64, t1444: f64, t6752: f64, t13182: f64, t176: f64, t1821: f64, t493: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17261 = 8.0_f64 / 243.0_f64 * t13243;
    let t17262 = 8.0_f64 / 135.0_f64 * t13245;
    let t17263 = 8.0_f64 / 135.0_f64 * t13249;
    let t17264 = 8.0_f64 / 135.0_f64 * t13251;
    let t17265 = t10087 / 135.0_f64;
    let t17266 = 2.0_f64 / 135.0_f64 * t10089;
    let t17268 = 4.0_f64 / 27.0_f64 * t1444 * t6752;
    let t17272 = 4.0_f64 / 27.0_f64 * t493 * t13182 * t176 * t1821;
    (t17261, t17262, t17263, t17264, t17265, t17266, t17268, t17272)
}
