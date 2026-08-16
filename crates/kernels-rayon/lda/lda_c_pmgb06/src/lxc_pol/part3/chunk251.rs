//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 251/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk251(t5: f64, t330: f64, t760: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t761 = t330 * t760;
    let t763 = piecewise3(t6, 0.0_f64, 2.0_f64 / 3.0_f64 * t761);
    let t764 = -t760;
    (t761, t763, t764)
}
