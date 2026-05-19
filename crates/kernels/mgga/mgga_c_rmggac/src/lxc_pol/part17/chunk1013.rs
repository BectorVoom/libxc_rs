//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1013/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1013<F: Float>(t45730: F, t5271: F, t1614: F, t40983: F, t46634: F, t46642: F, t46646: F, t46648: F, t46650: F, t46652: F, t46656: F, t46658: F, t46660: F, t46662: F, t5148: F, t5155: F, t570: F, t8946: F) -> F {
    let t46664 = t5271 * t45730;
    let t46666 = F::cast_from(0.15965655602485078085e0_f64) * t46634 + F::cast_from(0.47896966807455234256e0_f64) * t5155 * t8946 * t1614 - F::cast_from(0.23948483403727617128e0_f64) * t5148 * t40983 * t570 + F::cast_from(0.54549323308490683456e-1_f64) * t46642 + F::cast_from(0.18183107769496894486e-1_f64) * t46646 + F::cast_from(0.18183107769496894485e0_f64) * t46648 - F::cast_from(0.10227998120342003148e-1_f64) * t46650 + F::cast_from(0.13637330827122670864e-1_f64) * t46652 + F::cast_from(0.34093327067806677161e-2_f64) * t46656 + F::cast_from(0.35922725105591425692e0_f64) * t46658 - F::cast_from(0.8980681276397856423e0_f64) * t46660 - F::cast_from(0.17961362552795712846e0_f64) * t46662 - F::cast_from(0.17961362552795712846e0_f64) * t46664;
    t46666
}
