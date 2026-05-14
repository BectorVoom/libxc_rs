//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1227/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1227<F: Float>(t20567: F, t448: F, t17092: F, t5068: F, t16840: F, t5109: F, t1149: F, t6439: F, t3433: F, t1733: F, t5104: F, t3384: F, t6474: F, t12248: F, t12297: F, t12397: F, t16706: F, t16708: F, t17010: F, t17011: F, t20283: F, t20285: F, t20287: F, t20290: F, t20295: F, t20300: F, t20304: F, t20308: F, t20312: F, t20315: F, t20320: F) -> (F, F, F, F, F, F, F) {
    let t20568 = t20567 * t448;
    let t20571 = 4.0 * t17092 * t5068;
    let t20573 = 0.32163958997385070134e2 * t16840 * t5109;
    let t20574 = t6439 * t1149;
    let t20576 = 6.0 * t3433 * t20574;
    let t20577 = t1733 * t5104;
    let t20579 = 4.0 * t3384 * t20577;
    let t20580 = t6474 * t1149;
    let t20582 = 0.96491876992155210402e2 * t12248 * t20580;
    let t20597 = -t12397 + 0.76103703703703703703e-2 * t12297 + 0.1522074074074074074e-1 * t16706 + 0.761037037037037037e-2 * t16708 - t17010 - t17011 + 0.3805185185185185185e-2 * t20283 + 0.19025925925925925925e-1 * t20295 - 0.68493333333333333331e-1 * t20300 - 0.2283111111111111111e-1 * t20304 - 0.11415555555555555555e-1 * t20285 + 0.10274e0 * t20308 + 0.68493333333333333332e-1 * t20312 - 0.57077777777777777777e-2 * t20287 - 0.11415555555555555555e-1 * t20315 + 0.34246666666666666666e-1 * t20320 + 0.17123333333333333333e-1 * t20290;
    (t20568, t20571, t20573, t20576, t20579, t20582, t20597)
}
