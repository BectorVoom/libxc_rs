//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1461/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1461(t18754: f64, t360: f64, t6967: f64, t947: f64, t6970: f64, t18725: f64, t18729: f64, t18732: f64, t18735: f64, t18737: f64, t18741: f64, t18745: f64, t18748: f64, t18750: f64, t18752: f64) -> f64 {
    let t18755 = t360 * t18754;
    let t18757 = t6967 * t947;
    let t18759 = t6970 * t947;
    let t18761 = -0.48968_f64 * t18725 - t18729 + t18732 + t18735 - t360 * t18737 / 2.0_f64 + 3.0_f64 * t360 * t18741 - 2.0_f64 / 9.0_f64 * t18745 + t18748 - t18750 + t18752 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t18755 + 3.91744_f64 * t18757 - 0.97936_f64 * t18759;
    t18761
}
