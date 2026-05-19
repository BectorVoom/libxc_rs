//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1103/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1103<F: Float>(t10992: F, t11563: F, t2315: F, t3446: F, t10648: F, t10649: F, t2482: F, t58: F, t597: F, t37359: F, t37362: F, t37366: F, t37370: F, t37374: F, t39181: F, t39186: F, t39215: F, t39218: F, t39221: F, t39225: F, t39229: F, t39233: F) -> F {
    let t39239 = t3446 * t10992 * t11563 * t2315;
    let t39244 = t10648 * t10649 * t58 * t2482 * t597;
    let t39245 = F::cast_from(0.72042316457491791906e-3_f64) * t39244;
    let t39246 = -F::cast_from(0.72042316457491791906e-3_f64) * t39215 + F::cast_from(0.10248087766267884742e-3_f64) * t39218 + t37359 + F::cast_from(0.20496175532535769484e-3_f64) * t37362 + F::cast_from(0.36021158228745895953e-3_f64) * t39221 - F::cast_from(0.72042316457491791906e-3_f64) * t39225 + F::cast_from(0.10248087766267884742e-3_f64) * t39229 - F::cast_from(0.14408463291498358381e-2_f64) * t37366 - F::cast_from(0.19211284388664477842e-2_f64) * t39233 - F::cast_from(0.72042316457491791906e-3_f64) * t37370 + F::cast_from(0.72042316457491791906e-3_f64) * t37374 + F::cast_from(0.30487649791575028314e-3_f64) * t39239 - t39181 - t39186 + t39245;
    t39246
}
