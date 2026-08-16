//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1261/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1261<F: Float>(t40518: F, t37499: F, t37503: F, t37507: F, t39091: F, t39092: F, t39093: F, t39094: F, t40511: F, t40513: F, t40515: F, t40521: F, t42136: F, t42138: F, t42140: F) -> F {
    let t42229 = F::cast_from(0.60975299583150056624e-3_f64) * t40518;
    let t42231 = -F::cast_from(0.10248087766267884741e-3_f64) * t37499 + F::cast_from(0.1440846329149835838e-2_f64) * t37503 - F::cast_from(0.20496175532535769482e-3_f64) * t37507 - t42136 + t42138 - t42140 + t39091 - t39092 + t39093 - t39094 - F::cast_from(0.72042316457491791901e-3_f64) * t40511 - F::cast_from(0.30487649791575028312e-3_f64) * t40513 + F::cast_from(0.30487649791575028312e-3_f64) * t40515 - t42229 - F::cast_from(0.1440846329149835838e-2_f64) * t40521;
    t42231
}
