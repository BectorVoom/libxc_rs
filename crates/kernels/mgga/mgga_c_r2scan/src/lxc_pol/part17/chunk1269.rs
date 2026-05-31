//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1269/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1269<F: Float>(t12219: F, t40282: F, t11189: F, t3262: F, t42819: F, t3579: F, t41298: F, t37362: F, t37366: F, t37370: F, t37374: F, t39046: F, t39215: F, t39218: F, t39221: F, t39225: F, t39229: F, t39233: F, t44014: F, t44017: F, t44020: F, t44024: F) -> (F, F, F, F) {
    let t44937 = F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t40282 * t12219;
    let t44940 = F::cast_from(135.0_f64) / F::cast_from(64.0_f64) * t3262 * t11189 * t42819;
    let t44942 = t3579 * t41298 / F::cast_from(2.0_f64);
    let t44953 = -F::cast_from(0.2881692658299671676e-2_f64) * t39215 + F::cast_from(0.40992351065071538964e-3_f64) * t39218 + t39046 + F::cast_from(0.20496175532535769483e-3_f64) * t37362 + F::cast_from(0.1440846329149835838e-2_f64) * t39221 - F::cast_from(0.2881692658299671676e-2_f64) * t39225 + F::cast_from(0.40992351065071538964e-3_f64) * t39229 - F::cast_from(0.1440846329149835838e-2_f64) * t37366 - F::cast_from(0.7684513755465791136e-2_f64) * t39233 - F::cast_from(0.72042316457491791901e-3_f64) * t37370 + F::cast_from(0.72042316457491791901e-3_f64) * t37374 + t44014 + t44017 - t44020 - t44024;
    (t44937, t44940, t44942, t44953)
}
