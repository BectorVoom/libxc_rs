//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1142/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1142(t11110: f64, t11112: f64, t11115: f64, t11117: f64, t11119: f64, t11122: f64, t11124: f64, t8692: f64, t8693: f64, t8723: f64, t8724: f64, t8727: f64, t8733: f64, t8737: f64, t8738: f64, t8743: f64, t8746: f64) -> f64 {
    let t15006 = t8692 - 1.1696447245269292_f64 * t8693 - t8723 + 207.79030926817757_f64 * t8724 - 24.0_f64 * t11110 + 8.0_f64 * t11112 + t8727 + t8733 - 0.06506148780181044_f64 * t11115 - 0.04337432520120696_f64 * t11117 + 0.9631946627535314_f64 * t11119 + 0.04337432520120696_f64 * t11122 + 0.03253074390090522_f64 * t11124 - t8737 - 7.017868347161575_f64 * t8738 - t8743 + t8746;
    t15006
}
