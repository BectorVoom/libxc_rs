//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1102/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1102<F: Float>(t3275: F, t3472: F, t39300: F, t11465: F, t40276: F, t37362: F, t37366: F, t37370: F, t37374: F, t39046: F, t39215: F, t39218: F, t39221: F, t39225: F, t39229: F, t39233: F, t39239: F, t41153: F, t41156: F, t41158: F) -> (F, F, F) {
    let t42146 = 5.0 / 8.0 * t3275 * t3472 * t39300;
    let t42148 = 5.0 / 8.0 * t40276 * t11465;
    let t42161 = -0.1440846329149835838e-2 * t39215 + 0.20496175532535769482e-3 * t39218 + t39046 + 0.40992351065071538965e-3 * t37362 + 0.72042316457491791901e-3 * t39221 - 0.1440846329149835838e-2 * t39225 + 0.20496175532535769482e-3 * t39229 - 0.2881692658299671676e-2 * t37366 - 0.3842256877732895568e-2 * t39233 - 0.1440846329149835838e-2 * t37370 + 0.1440846329149835838e-2 * t37374 + t41153 + t41156 - t41158 + 0.60975299583150056624e-3 * t39239;
    (t42146, t42148, t42161)
}
