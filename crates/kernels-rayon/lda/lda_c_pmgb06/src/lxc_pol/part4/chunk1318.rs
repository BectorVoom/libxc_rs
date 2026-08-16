//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1318/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1318(t13345: f64, t13347: f64, t13370: f64, t13372: f64, t13374: f64, t13376: f64, t13379: f64, t13382: f64, t17080: f64, t17107: f64, t9502: f64, t9577: f64) -> f64 {
    let t17333 = -0.01679259259259259_f64 * t17080 - 0.0008396296296296296_f64 * t13345 - 0.0013993827160493828_f64 * t13347 - 0.006717037037037037_f64 * t13370 + 0.002239012345679012_f64 * t13372 + 0.002518888888888889_f64 * t13374 - 0.010075555555555556_f64 * t13376 + 0.005037777777777778_f64 * t13379 + 0.002518888888888889_f64 * t13382 - 0.0016792592592592592_f64 * t9577 - t9502 - 0.04534_f64 * t17107;
    t17333
}
