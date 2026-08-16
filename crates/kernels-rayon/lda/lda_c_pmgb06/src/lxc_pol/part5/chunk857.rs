//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 857/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk857(t1121: f64, t410: f64, t980: f64, t110: f64, t3705: f64, t1180: f64, t698: f64, t1025: f64, t3675: f64, t3868: f64, t633: f64, t964: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8583 = 0.08674865040241392_f64 * t1121 * t410 * t980;
    let t8586 = 0.13012297560362088_f64 * t1121 * t110 * t3705;
    let t8589 = 0.06747117253521083_f64 * t1121 * t1180 * t698;
    let t8594 = 578.9512619529313_f64 * t3675 * t3868 * t1025;
    let t8595 = t1025 * t1025;
    let t8598 = 24.0_f64 * t3675 * t8595 * t633;
    let t8599 = t964 * t964;
    (t8583, t8586, t8589, t8594, t8595, t8598, t8599)
}
