//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1227/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1227(t39752: f64, t39753: f64, t39770: f64, t39772: f64, t41542: f64, t41552: f64, t43230: f64, t43232: f64, t43234: f64, t43238: f64, t43240: f64, t43242: f64) -> f64 {
    let t44297 = -t39752 - t39753 - t41542 + 0.39029762157531132073e-1_f64 * t43230 + 0.87327386630866483588e-2_f64 * t43232 + 0.51220160311720645767e0_f64 * t39770 + 0.25610080155860322883e0_f64 * t43234 - 0.17073386770573548589e1_f64 * t39772 + 0.23115257973478049502e0_f64 * t43238 + t41552 + 0.43663693315433241794e-2_f64 * t43240 - 0.87327386630866483588e-2_f64 * t43242;
    t44297
}
