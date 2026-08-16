//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1145/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1145(t11036: f64, t9657: f64, t1070: f64, t31764: f64, t2928: f64, t37028: f64, t11033: f64, t2938: f64, t3366: f64, t9640: f64, t37039: f64, t37041: f64, t37066: f64, t37076: f64, t40822: f64, t40841: f64, t40845: f64, t41872: f64, t42524: f64, t42526: f64, t42528: f64, t42530: f64, t42532: f64) -> f64 {
    let t42534 = t11036 * t9657;
    let t42536 = t31764 * t1070;
    let t42539 = t37028 * t2928;
    let t42541 = t11033 * t2938;
    let t42543 = t9640 * t3366;
    let t42546 = -3.0_f64 / 4.0_f64 * t42524 + t42526 / 4.0_f64 + t42528 / 8.0_f64 + t41872 + t40822 + 3.0_f64 / 4.0_f64 * t42530 - t42532 / 2.0_f64 - t42534 / 4.0_f64 - t42536 / 8.0_f64 + 11.0_f64 / 9.0_f64 * t37041 + 2.0_f64 / 3.0_f64 * t42539 + t37039 - t40841 + t40845 - t42541 / 3.0_f64 + t42543 / 3.0_f64 - 11.0_f64 / 9.0_f64 * t37066 + t37076;
    t42546
}
