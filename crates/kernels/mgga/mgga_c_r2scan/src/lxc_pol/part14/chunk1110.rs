//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1110/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1110<F: Float>(t37499: F, t37503: F, t37507: F, t39091: F, t39092: F, t39093: F, t39094: F, t40511: F, t40513: F, t40515: F, t40521: F, t42136: F, t42138: F, t42140: F, t42229: F, t12198: F, t3270: F) -> (F, F) {
    let t42231 = -0.10248087766267884741e-3 * t37499 + 0.1440846329149835838e-2 * t37503 - 0.20496175532535769482e-3 * t37507 - t42136 + t42138 - t42140 + t39091 - t39092 + t39093 - t39094 - 0.72042316457491791901e-3 * t40511 - 0.30487649791575028312e-3 * t40513 + 0.30487649791575028312e-3 * t40515 - t42229 - 0.1440846329149835838e-2 * t40521;
    let t42234 = t3270 * t12198;
    (t42231, t42234)
}
