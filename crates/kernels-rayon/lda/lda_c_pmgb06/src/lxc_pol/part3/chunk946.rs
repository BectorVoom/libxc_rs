//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 946/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk946(t4516: f64, t643: f64, t4481: f64, t638: f64, t8545: f64, t8548: f64, t8552: f64, t8553: f64, t8555: f64, t8559: f64, t8560: f64, t8564: f64, t8567: f64, t8570: f64, t8572: f64, t8576: f64, t8580: f64, t8583: f64, t8586: f64) -> f64 {
    let t11070 = t643 * t4516;
    let t11073 = 24.0_f64 * t638 * t4481;
    let t11081 = -12.0_f64 * t11070 + t11073 - 36.0_f64 * t8545 + 3.0_f64 * t8548 - t8552 + 0.04879611585135783_f64 * t8553 - 0.09759223170271566_f64 * t8555 - t8559 - 0.06506148780181044_f64 * t8560 - t8564 - t8567 + t8570 + 0.03253074390090522_f64 * t8572 + 1.4447919941302971_f64 * t8576 + t8580 + t8583 + t8586;
    t11081
}
