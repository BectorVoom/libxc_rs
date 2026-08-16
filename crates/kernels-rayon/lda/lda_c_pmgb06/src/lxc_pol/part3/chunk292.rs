//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 292/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk292(t698: f64, t971: f64, t27: f64, t653: f64, t693: f64, t278: f64, t674: f64) -> (f64, f64, f64, f64, f64) {
    let t972 = t971 * t698;
    let t974 = t653 * t27;
    let t975 = t974 * t693;
    let t977 = t674 * t278;
    let t978 = 1.0_f64 / t977;
    (t972, t974, t975, t977, t978)
}
