//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1103/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1103(t10992: f64, t11563: f64, t2315: f64, t3446: f64, t10648: f64, t10649: f64, t2482: f64, t58: f64, t597: f64, t37359: f64, t37362: f64, t37366: f64, t37370: f64, t37374: f64, t39181: f64, t39186: f64, t39215: f64, t39218: f64, t39221: f64, t39225: f64, t39229: f64, t39233: f64) -> f64 {
    let t39239 = t3446 * t10992 * t11563 * t2315;
    let t39244 = t10648 * t10649 * t58 * t2482 * t597;
    let t39245 = 0.72042316457491791906e-3_f64 * t39244;
    let t39246 = -0.72042316457491791906e-3_f64 * t39215 + 0.10248087766267884742e-3_f64 * t39218 + t37359 + 0.20496175532535769484e-3_f64 * t37362 + 0.36021158228745895953e-3_f64 * t39221 - 0.72042316457491791906e-3_f64 * t39225 + 0.10248087766267884742e-3_f64 * t39229 - 0.14408463291498358381e-2_f64 * t37366 - 0.19211284388664477842e-2_f64 * t39233 - 0.72042316457491791906e-3_f64 * t37370 + 0.72042316457491791906e-3_f64 * t37374 + 0.30487649791575028314e-3_f64 * t39239 - t39181 - t39186 + t39245;
    t39246
}
