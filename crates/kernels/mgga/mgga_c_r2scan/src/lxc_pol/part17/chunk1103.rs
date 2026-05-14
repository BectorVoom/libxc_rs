//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1103/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1103<F: Float>(t3579: F, t41298: F, t37362: F, t37366: F, t37370: F, t37374: F, t39046: F, t39215: F, t39218: F, t39221: F, t39225: F, t39229: F, t39233: F, t44014: F, t44017: F, t44020: F, t44024: F) -> (F, F) {
    let t44942 = t3579 * t41298 / 2.0;
    let t44953 = -0.2881692658299671676e-2 * t39215 + 0.40992351065071538964e-3 * t39218 + t39046 + 0.20496175532535769483e-3 * t37362 + 0.1440846329149835838e-2 * t39221 - 0.2881692658299671676e-2 * t39225 + 0.40992351065071538964e-3 * t39229 - 0.1440846329149835838e-2 * t37366 - 0.7684513755465791136e-2 * t39233 - 0.72042316457491791901e-3 * t37370 + 0.72042316457491791901e-3 * t37374 + t44014 + t44017 - t44020 - t44024;
    (t44942, t44953)
}
