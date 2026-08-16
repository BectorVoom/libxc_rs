//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 835/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk835(t44: f64, t7959: f64, t7967: f64, t224: f64, t3007: f64, t4070: f64, t7687: f64, t7689: f64, t7692: f64, t7694: f64, t7698: f64, t7700: f64, t7701: f64, t7702: f64, t7703: f64, t7707: f64, t7708: f64, t7713: f64, t7717: f64) -> (f64, f64) {
    let t7970 = (t7959 / 2.0_f64 + t7967 / 2.0_f64) * t44;
    let t7973 = t7687 + t7689 + t7692 + t7694 + t7698 - t7970 * t224 / 15.0_f64 + t7700 + t7701 - t7702 - t7703 - t7707 - t7708 + t3007 + t7713 + t7717 + t4070;
    (t7970, t7973)
}
