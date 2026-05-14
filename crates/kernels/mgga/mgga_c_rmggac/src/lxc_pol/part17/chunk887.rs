//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 887/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk887<F: Float>(t27041: F, t45568: F, t27091: F, t45572: F, t25877: F, t45577: F, t45730: F, t5271: F, t1614: F, t40983: F, t46634: F, t46642: F, t46646: F, t46648: F, t46650: F, t46652: F, t46656: F, t5148: F, t5155: F, t570: F, t8946: F) -> (F,) {
    let t46658 = t27041 * t45568;
    let t46660 = t27091 * t45572;
    let t46662 = t25877 * t45577;
    let t46664 = t5271 * t45730;
    let t46666 = 0.15965655602485078085e0 * t46634 + 0.47896966807455234256e0 * t5155 * t8946 * t1614 - 0.23948483403727617128e0 * t5148 * t40983 * t570 + 0.54549323308490683456e-1 * t46642 + 0.18183107769496894486e-1 * t46646 + 0.18183107769496894485e0 * t46648 - 0.10227998120342003148e-1 * t46650 + 0.13637330827122670864e-1 * t46652 + 0.34093327067806677161e-2 * t46656 + 0.35922725105591425692e0 * t46658 - 0.8980681276397856423e0 * t46660 - 0.17961362552795712846e0 * t46662 - 0.17961362552795712846e0 * t46664;
    (t46666,)
}
