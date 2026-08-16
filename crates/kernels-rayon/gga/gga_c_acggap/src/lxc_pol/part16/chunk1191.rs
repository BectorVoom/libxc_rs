//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1191/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1191(t1165: f64, t5697: f64, t7575: f64, t8600: f64, t31693: f64, t31700: f64, t31702: f64, t31704: f64, t31708: f64, t31721: f64, t36042: f64, t36066: f64, t36082: f64, t36086: f64, t36088: f64, t36090: f64, t36097: f64, t36119: f64, t36123: f64, t37858: f64, t37869: f64) -> f64 {
    let t40418 = t7575 * t1165 * t8600 * t5697;
    let t40422 = -t36042 - t36066 + 0.7145669686344956162e-3_f64 * t31693 - 0.14291339372689912324e-3_f64 * t31700 + 0.15724046144802076034e-3_f64 * t31702 + 0.20965394859736101378e-3_f64 * t31704 + 0.28582678745379824648e-3_f64 * t31708 + t36082 - t31721 + t37858 + 0.18868855373762491241e-1_f64 * t40418 + t36086 + t36088 - t36090 + t36097 - t37869 + 0.41930789719472202756e-3_f64 * t36119 - 0.62896184579208304134e-3_f64 * t36123;
    t40422
}
