//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1261/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1261(t40518: f64, t37499: f64, t37503: f64, t37507: f64, t39091: f64, t39092: f64, t39093: f64, t39094: f64, t40511: f64, t40513: f64, t40515: f64, t40521: f64, t42136: f64, t42138: f64, t42140: f64) -> f64 {
    let t42229 = 0.60975299583150056624e-3_f64 * t40518;
    let t42231 = -0.10248087766267884741e-3_f64 * t37499 + 0.1440846329149835838e-2_f64 * t37503 - 0.20496175532535769482e-3_f64 * t37507 - t42136 + t42138 - t42140 + t39091 - t39092 + t39093 - t39094 - 0.72042316457491791901e-3_f64 * t40511 - 0.30487649791575028312e-3_f64 * t40513 + 0.30487649791575028312e-3_f64 * t40515 - t42229 - 0.1440846329149835838e-2_f64 * t40521;
    t42231
}
