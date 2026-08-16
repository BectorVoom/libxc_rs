//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1251/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1251(t10687: f64, t12056: f64, t3275: f64, t3472: f64, t39300: f64, t11465: f64, t40276: f64, t37362: f64, t37366: f64, t37370: f64, t37374: f64, t39046: f64, t39215: f64, t39218: f64, t39221: f64, t39225: f64, t39229: f64, t39233: f64, t39239: f64, t41153: f64, t41156: f64, t41158: f64) -> (f64, f64, f64, f64) {
    let t42143 = t3275 * t12056 * t10687 / 4.0_f64;
    let t42146 = 5.0_f64 / 8.0_f64 * t3275 * t3472 * t39300;
    let t42148 = 5.0_f64 / 8.0_f64 * t40276 * t11465;
    let t42161 = -0.1440846329149835838e-2_f64 * t39215 + 0.20496175532535769482e-3_f64 * t39218 + t39046 + 0.40992351065071538965e-3_f64 * t37362 + 0.72042316457491791901e-3_f64 * t39221 - 0.1440846329149835838e-2_f64 * t39225 + 0.20496175532535769482e-3_f64 * t39229 - 0.2881692658299671676e-2_f64 * t37366 - 0.3842256877732895568e-2_f64 * t39233 - 0.1440846329149835838e-2_f64 * t37370 + 0.1440846329149835838e-2_f64 * t37374 + t41153 + t41156 - t41158 + 0.60975299583150056624e-3_f64 * t39239;
    (t42143, t42146, t42148, t42161)
}
