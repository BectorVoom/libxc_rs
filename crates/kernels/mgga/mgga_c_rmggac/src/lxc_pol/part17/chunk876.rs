//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 876/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk876<F: Float>(t262: F, t46261: F, t7785: F, t352: F, t9872: F, t7788: F, t2350: F, t5144: F, t25820: F, t5267: F, t25877: F, t35922: F, t35926: F, t41115: F, t46413: F, t46417: F, t46421: F, t46425: F, t46429: F, t46432: F) -> (F, F, F, F, F, F) {
    let t46434 = t262 * t46261;
    let t46435 = t7785 * t46434;
    let t46437 = t9872 * t352;
    let t46438 = t262 * t46437;
    let t46439 = t7788 * t46438;
    let t46441 = t2350 * t5144;
    let t46442 = t25820 * t46441;
    let t46444 = t2350 * t5267;
    let t46445 = t25877 * t46444;
    let t46449 = -0.6818665413561335432e-1 * t46413 - 0.13637330827122670864e-1 * t46417 + 0.10227998120342003148e-1 * t46421 - 0.13637330827122670864e-1 * t46425 - 0.68186654135613354322e-2 * t46429 - 0.20455996240684006296e-1 * t46432 + 0.40911992481368012592e-1 * t46435 + 0.10227998120342003148e-1 * t46439 + 0.17961362552795712846e0 * t46442 - 0.35922725105591425692e0 * t46445 + 0.33335697577410973224e-1 * t35922 + 0.1333427903096438929e0 * t35926 + t41115;
    (t46434, t46437, t46438, t46441, t46444, t46449)
}
