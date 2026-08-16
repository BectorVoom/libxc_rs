//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 244/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk244(t259: f64, t119: f64, t20: f64, t268: f64, t272: f64, t1097: f64, t397: f64, t1111: f64, t275: f64, t918: f64, t278: f64, t1004: f64, t1101: f64, t282: f64, t939: f64, t977: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t270 = 0.0_f64 < t259;
    let t1118 = t119 * t20;
    let t1119 = t268 * t1118;
    let t1120 = t272 * t272;
    let t1121 = 1.0_f64 / t1120;
    let t1123 = piecewise3(t270, t1097, -t1097);
    let t1125 = t397 * t1121 * t1123;
    let t1128 = 0.5397236614853195164e-1_f64 * t1111 * t119 * t275 - 0.12593552101324122049e0_f64 * t268 * t918 * t275 - 0.5397236614853195164e-1_f64 * t1119 * t1125;
    let t1129 = 1.0_f64 / t278;
    let t1130 = t1128 * t1129;
    let t1136 = t1097 * t282 - 0.193e0_f64 * t1101 * t1130 - 0.92858888888888888886e-2_f64 * t939 + 0.69644166666666666665e-2_f64 * t977 - 0.69644166666666666665e-2_f64 * t1004;
    (t1118, t1119, t1120, t1121, t1123, t1125, t1128, t1129, t1130, t1136)
}
