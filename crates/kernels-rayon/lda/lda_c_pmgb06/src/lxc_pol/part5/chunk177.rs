//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 177/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk177(t12: f64, t465: f64, t477: f64, t137: f64, t132: f64, t337: f64, t44: f64, t131: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t13 = t12 <= zeta_threshold;
    let t478 = t465 * t477;
    let t479 = t137 * t478;
    let t481 = t132 * t479 / 30.0_f64;
    let t484 = piecewise3(t13, 0.0_f64, 2.0_f64 * t12 * t337);
    let t485 = t484 * t44;
    let t486 = t485 * t131;
    (t478, t479, t481, t485, t486)
}
