//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1212/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1212(t19284: f64, t19286: f64, t19289: f64, t19291: f64, t19293: f64, t19295: f64, t19298: f64, t19300: f64, t19302: f64, t19307: f64, t19309: f64, t21873: f64, t21891: f64, t224: f64, t44: f64) -> f64 {
    let t21897 = -t19284 - t19286 - t19289 + t19291 + t19293 - t19295 - t19298 + t19300 + t19302 - t19307 + t19309 - (t21873 / 2.0_f64 + t21891 / 2.0_f64) * t44 * t224 / 15.0_f64;
    t21897
}
