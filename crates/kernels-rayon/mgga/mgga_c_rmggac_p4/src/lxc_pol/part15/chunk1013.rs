//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1013/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1013(t45730: f64, t5271: f64, t1614: f64, t40983: f64, t46634: f64, t46642: f64, t46646: f64, t46648: f64, t46650: f64, t46652: f64, t46656: f64, t46658: f64, t46660: f64, t46662: f64, t5148: f64, t5155: f64, t570: f64, t8946: f64) -> f64 {
    let t46664 = t5271 * t45730;
    let t46666 = 0.15965655602485078085e0_f64 * t46634 + 0.47896966807455234256e0_f64 * t5155 * t8946 * t1614 - 0.23948483403727617128e0_f64 * t5148 * t40983 * t570 + 0.54549323308490683456e-1_f64 * t46642 + 0.18183107769496894486e-1_f64 * t46646 + 0.18183107769496894485e0_f64 * t46648 - 0.10227998120342003148e-1_f64 * t46650 + 0.13637330827122670864e-1_f64 * t46652 + 0.34093327067806677161e-2_f64 * t46656 + 0.35922725105591425692e0_f64 * t46658 - 0.8980681276397856423e0_f64 * t46660 - 0.17961362552795712846e0_f64 * t46662 - 0.17961362552795712846e0_f64 * t46664;
    t46666
}
