//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1021/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1021(t6443: f64, t802: f64, t11758: f64, t14311: f64, t14312: f64, t14314: f64, t14316: f64, t19209: f64, t19211: f64, t19215: f64, t19217: f64, t19219: f64) -> (f64, f64) {
    let t19221 = t802 * t6443 / 5.0_f64;
    let t19222 = -t19209 + t19211 + t14311 + 0.6492624817418906_f64 * t14312 - 0.2885611029963958_f64 * t14314 - 0.03354522822333102_f64 * t14316 - t19215 - t19217 + t19219 + t19221 + t11758;
    (t19221, t19222)
}
