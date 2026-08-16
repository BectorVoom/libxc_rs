//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1209/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1209(t39548: f64, t39558: f64, t37660: f64, t39540: f64, t39542: f64, t39545: f64, t39550: f64, t39552: f64, t39554: f64, t39561: f64, t39563: f64, t39565: f64) -> f64 {
    let t41435 = 0.95219938395347901946e-2_f64 * t39548;
    let t41439 = 0.45022119329691164871e0_f64 * t39558;
    let t41443 = -0.87327386630866483588e-2_f64 * t39540 - 0.32927245914677557992e0_f64 * t39542 - 0.52009330440325611378e0_f64 * t39545 - 0.28565981518604370584e-1_f64 * t37660 - t41435 - 0.10975748638225852664e0_f64 * t39550 - 0.86682217400542685632e-1_f64 * t39552 - 0.17336443480108537126e0_f64 * t39554 - t41439 - 0.86682217400542685632e-1_f64 * t39561 - 0.2600466522016280569e0_f64 * t39563 - 0.5200933044032561138e0_f64 * t39565;
    t41443
}
