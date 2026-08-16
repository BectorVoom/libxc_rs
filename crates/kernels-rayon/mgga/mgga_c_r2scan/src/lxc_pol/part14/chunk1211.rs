//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1211/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1211(t39601: f64, t39607: f64, t37702: f64, t37707: f64, t37714: f64, t39599: f64, t39604: f64, t39610: f64, t39616: f64, t39619: f64, t39622: f64, t39627: f64) -> (f64, f64) {
    let t41464 = 0.10975748638225852664e-1_f64 * t39601;
    let t41466 = 0.93149212406257582492e-1_f64 * t39607;
    let t41471 = -0.19514881078765566037e-1_f64 * t37702 - 0.90044238659382329742e0_f64 * t37707 - 0.95219938395347901946e-2_f64 * t37714 + 0.43663693315433241794e-2_f64 * t39599 + t41464 - 0.87327386630866483588e-2_f64 * t39604 - t41466 - 0.17336443480108537126e0_f64 * t39610 - 0.43902994552903410656e0_f64 * t39616 - 0.34672886960217074252e0_f64 * t39619 - 0.10401866088065122276e1_f64 * t39622;
    let t41474 = 0.46230515946956099004e0_f64 * t39627;
    (t41471, t41474)
}
