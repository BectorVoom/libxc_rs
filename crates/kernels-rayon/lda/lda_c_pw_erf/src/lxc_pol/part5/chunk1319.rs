//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1319/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1319(t611: f64, t7827: f64, t12508: f64, t12509: f64, t12558: f64, t12638: f64, t20179: f64, t21325: f64, t21326: f64, t21327: f64, t21329: f64, t21330: f64, t21331: f64, t21333: f64, t225: f64, t231: f64) -> f64 {
    let t23236 = t7827 * t611;
    let t23238 = t21325 + t12508 + 4.0_f64 * t12509 + 4.0_f64 / 3.0_f64 * t20179 * t225 * t231 + 4.0_f64 / 3.0_f64 * t23236 + t21326 - t21327 - t21329 - t12558 + t21330 + t21331 - t12638 - t21333;
    t23238
}
