//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1199/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1199(t27: f64, t693: f64, t7402: f64, t8545: f64, t8548: f64, t8552: f64, t8553: f64, t8555: f64, t8559: f64, t8560: f64, t8564: f64, t8567: f64, t8570: f64, t8576: f64, t8580: f64, t8583: f64, t8586: f64, t8589: f64, t8594: f64) -> f64 {
    let t21727 = t7402 * t27 * t693;
    let t21729 = -12.0_f64 * t8545 + t8548 - t8552 + 0.01626537195045261_f64 * t8553 - 0.03253074390090522_f64 * t8555 - t8559 - 0.02168716260060348_f64 * t8560 - t8564 - t8567 + t8570 + 0.4815973313767657_f64 * t8576 + t8580 + t8583 + t8586 + t8589 - t8594 - 0.00018311447306006544_f64 * t21727;
    t21729
}
