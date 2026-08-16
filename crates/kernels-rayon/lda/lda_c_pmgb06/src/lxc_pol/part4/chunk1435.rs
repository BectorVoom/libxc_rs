//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1435/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1435(t17794: f64, t17797: f64, t17800: f64, t17802: f64, t17804: f64, t17806: f64, t17808: f64, t17810: f64, t17812: f64, t17815: f64, t17819: f64, t17822: f64, t17824: f64, t17828: f64, t18355: f64, t18377: f64, t224: f64, t44: f64) -> f64 {
    let t18383 = -(t18355 / 2.0_f64 + t18377 / 2.0_f64) * t44 * t224 / 15.0_f64 - t17794 - t17797 + t17800 - t17802 - t17804 - t17806 - t17808 + t17810 - t17812 - t17815 - t17819 + t17822 - t17824 - t17828;
    t18383
}
