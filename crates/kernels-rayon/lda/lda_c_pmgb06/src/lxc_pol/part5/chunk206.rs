//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 206/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk206(t12: f64, t337: f64, t598: f64, t44: f64, t597: f64, t131: f64, t34: f64, zeta_threshold: f64) -> (f64, f64) {
    let t13 = t12 <= zeta_threshold;
    let t601 = piecewise3(t13, 0.0_f64, 8.0_f64 / 3.0_f64 * t598 * t337);
    let t604 = (t597 / 2.0_f64 + t601 / 2.0_f64) * t44;
    let t607 = t131 * t34;
    (t604, t607)
}
