//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 244/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk244<F: Float>(t259: F, t119: F, t20: F, t268: F, t272: F, t1097: F, t397: F, t1111: F, t275: F, t918: F, t278: F, t1004: F, t1101: F, t282: F, t939: F, t977: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t270 = F::cast_from(0.0_f64) < t259;
    let t1118 = t119 * t20;
    let t1119 = t268 * t1118;
    let t1120 = t272 * t272;
    let t1121 = F::cast_from(1.0_f64) / t1120;
    let t1123 = piecewise3::<F>(t270, t1097, -t1097);
    let t1125 = t397 * t1121 * t1123;
    let t1128 = F::cast_from(0.5397236614853195164e-1_f64) * t1111 * t119 * t275 - F::cast_from(0.12593552101324122049e0_f64) * t268 * t918 * t275 - F::cast_from(0.5397236614853195164e-1_f64) * t1119 * t1125;
    let t1129 = F::cast_from(1.0_f64) / t278;
    let t1130 = t1128 * t1129;
    let t1136 = t1097 * t282 - F::cast_from(0.193e0_f64) * t1101 * t1130 - F::cast_from(0.92858888888888888886e-2_f64) * t939 + F::cast_from(0.69644166666666666665e-2_f64) * t977 - F::cast_from(0.69644166666666666665e-2_f64) * t1004;
    (t1118, t1119, t1120, t1121, t1123, t1125, t1128, t1129, t1130, t1136)
}
