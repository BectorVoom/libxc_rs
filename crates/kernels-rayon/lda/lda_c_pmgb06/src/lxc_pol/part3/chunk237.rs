//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 237/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk237(t110: f64, t269: f64, t282: f64, t30: f64, t619: f64, t636: f64, t661: f64, t668: f64, t676: f64, t683: f64) -> f64 {
    let t686 = 0.0005323764196666666_f64 * t30 * t110 * t269 + 1.0_f64 * t661 * t668 - t619 - t636 + 0.00018311447306006544_f64 * t30 * t110 * t282 + 0.5848223622634646_f64 * t676 * t683;
    t686
}
