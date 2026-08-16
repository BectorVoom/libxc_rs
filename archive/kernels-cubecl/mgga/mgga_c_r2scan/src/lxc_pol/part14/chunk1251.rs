//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1251/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1251<F: Float>(t10687: F, t12056: F, t3275: F, t3472: F, t39300: F, t11465: F, t40276: F, t37362: F, t37366: F, t37370: F, t37374: F, t39046: F, t39215: F, t39218: F, t39221: F, t39225: F, t39229: F, t39233: F, t39239: F, t41153: F, t41156: F, t41158: F) -> (F, F, F, F) {
    let t42143 = t3275 * t12056 * t10687 / F::cast_from(4.0_f64);
    let t42146 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t3275 * t3472 * t39300;
    let t42148 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t40276 * t11465;
    let t42161 = -F::cast_from(0.1440846329149835838e-2_f64) * t39215 + F::cast_from(0.20496175532535769482e-3_f64) * t39218 + t39046 + F::cast_from(0.40992351065071538965e-3_f64) * t37362 + F::cast_from(0.72042316457491791901e-3_f64) * t39221 - F::cast_from(0.1440846329149835838e-2_f64) * t39225 + F::cast_from(0.20496175532535769482e-3_f64) * t39229 - F::cast_from(0.2881692658299671676e-2_f64) * t37366 - F::cast_from(0.3842256877732895568e-2_f64) * t39233 - F::cast_from(0.1440846329149835838e-2_f64) * t37370 + F::cast_from(0.1440846329149835838e-2_f64) * t37374 + t41153 + t41156 - t41158 + F::cast_from(0.60975299583150056624e-3_f64) * t39239;
    (t42143, t42146, t42148, t42161)
}
