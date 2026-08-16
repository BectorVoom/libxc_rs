//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 326/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk326(t322: f64, t1080: f64, t1081: f64, t1083: f64, t1085: f64, t1087: f64, t1089: f64, t1091: f64, t1096: f64, t343: f64, t352: f64, t855: f64, t259: f64, t869: f64) -> (f64, f64) {
    let t323 = t322 <= 0.0_f64;
    let t331 = t322 <= 0.25e1_f64;
    let t1100 = piecewise5(t323, t1080, t331, -0.64e0_f64 * t1081 - 0.8704e0_f64 * t1083 - 0.4607056813647e1_f64 * t1085 + 0.122462410087e2_f64 * t1087 - 0.957855118103e1_f64 * t1089 + 0.3101306810232e1_f64 * t1091 - 0.362942158544e0_f64 * t343 * t1081, -0.105e1_f64 * t855 * t1096 * t352);
    let t1102 = t869 * t259;
    (t1100, t1102)
}
