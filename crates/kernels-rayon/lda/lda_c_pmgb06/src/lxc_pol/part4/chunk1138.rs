//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1138/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1138(t11007: f64, t11065: f64, t14935: f64, t14939: f64, t14942: f64, t14944: f64, t14947: f64, t283: f64, t8482: f64, t8519: f64, t8520: f64, t8526: f64, t8529: f64, t8531: f64, t8534: f64, t8538: f64, t8541: f64, t8543: f64) -> f64 {
    let t14956 = 0.0197516734986138_f64 * t14935 * t283 + t8482 - t8519 + 0.01084358130030174_f64 * t14939 - 240.0_f64 * t8520 - 0.5848223622634646_f64 * t14942 - 17.315859105681465_f64 * t14944 - 1.1696447245269292_f64 * t14947 + t8526 - 24.0_f64 * t8529 + 32.0_f64 * t8531 + t8534 - 8.0_f64 * t8538 + 12.0_f64 * t8541 + 120.0_f64 * t8543 + 2.0_f64 * t11007 - 32.0_f64 * t11065;
    t14956
}
