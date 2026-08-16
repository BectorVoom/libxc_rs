//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1141/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1141(t1065: f64, t2395: f64, t248: f64, t11090: f64, t11092: f64, t11095: f64, t11097: f64, t11099: f64, t11101: f64, t8640: f64, t8644: f64, t8647: f64, t8651: f64, t8655: f64, t8659: f64, t8663: f64, t8668: f64, t8675: f64, t8684: f64, t8685: f64) -> f64 {
    let t14984 = t248 * t2395 * t1065;
    let t14993 = t8640 + t8644 - t8647 - t8651 + t8655 + t8659 + 0.00024415263074675396_f64 * t8663 + t8668 + t14984 + 120.0_f64 * t11090 + 80.0_f64 * t11092 - 48.0_f64 * t11095 + 96.0_f64 * t11097 + 160.0_f64 * t11099 - 240.0_f64 * t11101 + 1.1696447245269292_f64 * t8675 - t8684 - 2050.8037716432814_f64 * t8685;
    t14993
}
