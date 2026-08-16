//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2596/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2596(t10175: f64, t14090: f64, t14100: f64, t9671: f64, t1357: f64, t14269: f64, t689: f64, t1358: f64, t14066: f64, t212: f64, t13747: f64, t4071: f64, t46368: f64, t46369: f64, t46378: f64, t46381: f64, t46385: f64, t46388: f64, t47800: f64, t47802: f64, t47806: f64, t47808: f64, t47811: f64) -> f64 {
    let t47813 = t10175 * t14090;
    let t47814 = 0.39029762157531132076e-1_f64 * t47813;
    let t47816 = t14100 * t9671;
    let t47819 = t689 * t1357 * t14269;
    let t47825 = t689 * t212 * t14066 * t1358;
    let t47828 = -t46368 + 0.19514881078765566037e-2_f64 * t47800 + 0.17073386770573548589e-1_f64 * t47802 - 0.51220160311720645767e-1_f64 * t46369 - t47806 - 0.32927245914677557992e-1_f64 * t47808 + 0.32927245914677557992e-1_f64 * t47811 + t47814 + 0.58911598146606471822e-3_f64 * t46378 - 0.29272321618148349057e-1_f64 * t47816 + 0.16463622957338778996e-1_f64 * t47819 + 0.79025390195226139182e1_f64 * t4071 * t13747 - 0.16463622957338778996e-1_f64 * t47825 + 0.54878743191129263322e-2_f64 * t46381 - t46385 - t46388;
    t47828
}
