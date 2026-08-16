//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 999/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk999(t25820: f64, t46441: f64, t2350: f64, t5267: f64, t25877: f64, t35922: f64, t35926: f64, t41115: f64, t46413: f64, t46417: f64, t46421: f64, t46425: f64, t46429: f64, t46432: f64, t46435: f64, t46439: f64) -> (f64, f64) {
    let t46442 = t25820 * t46441;
    let t46444 = t2350 * t5267;
    let t46445 = t25877 * t46444;
    let t46449 = -0.6818665413561335432e-1_f64 * t46413 - 0.13637330827122670864e-1_f64 * t46417 + 0.10227998120342003148e-1_f64 * t46421 - 0.13637330827122670864e-1_f64 * t46425 - 0.68186654135613354322e-2_f64 * t46429 - 0.20455996240684006296e-1_f64 * t46432 + 0.40911992481368012592e-1_f64 * t46435 + 0.10227998120342003148e-1_f64 * t46439 + 0.17961362552795712846e0_f64 * t46442 - 0.35922725105591425692e0_f64 * t46445 + 0.33335697577410973224e-1_f64 * t35922 + 0.1333427903096438929e0_f64 * t35926 + t41115;
    (t46444, t46449)
}
