//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1092/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1092<F: Float>(t37359: F, t37362: F, t37366: F, t37370: F, t37374: F, t39215: F, t39218: F, t39221: F, t39225: F, t39229: F, t39233: F, t39239: F, t42383: F, t42387: F, t42391: F, t10935: F, t3162: F, t3446: F) -> (F, F) {
    let t43818 = -0.14408463291498358381e-2 * t39215 + 0.20496175532535769484e-3 * t39218 + t37359 + 0.10248087766267884742e-3 * t37362 + 0.72042316457491791906e-3 * t39221 - 0.14408463291498358381e-2 * t39225 + 0.20496175532535769484e-3 * t39229 - 0.72042316457491791906e-3 * t37366 - 0.38422568777328955684e-2 * t39233 - 0.36021158228745895953e-3 * t37370 + 0.36021158228745895953e-3 * t37374 + t42383 + 0.60975299583150056628e-3 * t39239 + t42387 - t42391;
    let t43820 = t3446 * t10935 * t3162;
    (t43818, t43820)
}
