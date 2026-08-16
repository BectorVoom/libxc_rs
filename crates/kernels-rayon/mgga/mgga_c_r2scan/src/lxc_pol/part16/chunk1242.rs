//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1242/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1242(t37359: f64, t37362: f64, t37366: f64, t37370: f64, t37374: f64, t39215: f64, t39218: f64, t39221: f64, t39225: f64, t39229: f64, t39233: f64, t39239: f64, t42383: f64, t42387: f64, t42391: f64) -> f64 {
    let t43818 = -0.14408463291498358381e-2_f64 * t39215 + 0.20496175532535769484e-3_f64 * t39218 + t37359 + 0.10248087766267884742e-3_f64 * t37362 + 0.72042316457491791906e-3_f64 * t39221 - 0.14408463291498358381e-2_f64 * t39225 + 0.20496175532535769484e-3_f64 * t39229 - 0.72042316457491791906e-3_f64 * t37366 - 0.38422568777328955684e-2_f64 * t39233 - 0.36021158228745895953e-3_f64 * t37370 + 0.36021158228745895953e-3_f64 * t37374 + t42383 + 0.60975299583150056628e-3_f64 * t39239 + t42387 - t42391;
    t43818
}
