//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 981/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk981(t39372: f64, t5148: f64, t40864: f64, t4669: f64, t118: f64, t2001: f64, t352: f64, t38523: f64, t7720: f64, t34884: f64, t9118: f64, t2283: f64, t34881: f64) -> (f64, f64, f64, f64, f64) {
    let t41560 = t5148 * t39372;
    let t41562 = t4669 * t40864;
    let t41576 = t2001 * t118 * t38523 * t352;
    let t41577 = t7720 * t41576;
    let t41579 = t34884 * t9118;
    let t41581 = t34881 * t2283;
    (t41560, t41562, t41577, t41579, t41581)
}
