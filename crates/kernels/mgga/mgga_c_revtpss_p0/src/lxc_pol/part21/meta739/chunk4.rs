//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2596/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2596<F: Float>(t10175: F, t14090: F, t14100: F, t9671: F, t1357: F, t14269: F, t689: F, t1358: F, t14066: F, t212: F, t13747: F, t4071: F, t46368: F, t46369: F, t46378: F, t46381: F, t46385: F, t46388: F, t47800: F, t47802: F, t47806: F, t47808: F, t47811: F) -> F {
    let t47813 = t10175 * t14090;
    let t47814 = F::cast_from(0.39029762157531132076e-1_f64) * t47813;
    let t47816 = t14100 * t9671;
    let t47819 = t689 * t1357 * t14269;
    let t47825 = t689 * t212 * t14066 * t1358;
    let t47828 = -t46368 + F::cast_from(0.19514881078765566037e-2_f64) * t47800 + F::cast_from(0.17073386770573548589e-1_f64) * t47802 - F::cast_from(0.51220160311720645767e-1_f64) * t46369 - t47806 - F::cast_from(0.32927245914677557992e-1_f64) * t47808 + F::cast_from(0.32927245914677557992e-1_f64) * t47811 + t47814 + F::cast_from(0.58911598146606471822e-3_f64) * t46378 - F::cast_from(0.29272321618148349057e-1_f64) * t47816 + F::cast_from(0.16463622957338778996e-1_f64) * t47819 + F::cast_from(0.79025390195226139182e1_f64) * t4071 * t13747 - F::cast_from(0.16463622957338778996e-1_f64) * t47825 + F::cast_from(0.54878743191129263322e-2_f64) * t46381 - t46385 - t46388;
    t47828
}
