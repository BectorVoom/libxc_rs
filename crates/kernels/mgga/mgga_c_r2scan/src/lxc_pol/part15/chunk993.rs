//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 993/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk993<F: Float>(t10648: F, t10649: F, t2482: F, t58: F, t597: F, t37359: F, t37362: F, t37366: F, t37370: F, t37374: F, t39181: F, t39186: F, t39215: F, t39218: F, t39221: F, t39225: F, t39229: F, t39233: F, t39239: F) -> (F,) {
    let t39244 = t10648 * t10649 * t58 * t2482 * t597;
    let t39245 = 0.72042316457491791906e-3 * t39244;
    let t39246 = -0.72042316457491791906e-3 * t39215 + 0.10248087766267884742e-3 * t39218 + t37359 + 0.20496175532535769484e-3 * t37362 + 0.36021158228745895953e-3 * t39221 - 0.72042316457491791906e-3 * t39225 + 0.10248087766267884742e-3 * t39229 - 0.14408463291498358381e-2 * t37366 - 0.19211284388664477842e-2 * t39233 - 0.72042316457491791906e-3 * t37370 + 0.72042316457491791906e-3 * t37374 + 0.30487649791575028314e-3 * t39239 - t39181 - t39186 + t39245;
    (t39246,)
}
