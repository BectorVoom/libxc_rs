//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1137/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1137(t1704: f64, t698: f64, t10252: f64, t118: f64, t1587: f64, t1614: f64, t25820: f64, t27101: f64, t333: f64, t352: f64, t41521: f64, t41522: f64, t44337: f64, t46748: f64, t46750: f64, t49210: f64, t49560: f64, t5148: f64, t5155: f64, t5259: f64, t6444: f64, t9523: f64, t9540: f64) -> f64 {
    let t49572 = t698 * t1704;
    let t49591 = 0.47896966807455234256e0_f64 * t5155 * t9540 * t1614 - 0.35922725105591425692e0_f64 * t25820 * t49572 * t333 + 0.11974241701863808564e0_f64 * t46748 - 0.23948483403727617128e0_f64 * t27101 * t49572 * t352 - 0.17961362552795712846e0_f64 * t46750 - 0.79828278012425390428e-1_f64 * t118 * t49210 + 0.23948483403727617128e0_f64 * t5259 * t9523 * t1587 + t41521 - t41522 + t44337 + 0.11974241701863808564e0_f64 * t6444 * t10252 - 0.23948483403727617128e0_f64 * t5148 * t49560 * t352;
    t49591
}
