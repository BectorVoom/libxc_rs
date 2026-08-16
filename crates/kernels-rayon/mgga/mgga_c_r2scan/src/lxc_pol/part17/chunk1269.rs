//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1269/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1269(t12219: f64, t40282: f64, t11189: f64, t3262: f64, t42819: f64, t3579: f64, t41298: f64, t37362: f64, t37366: f64, t37370: f64, t37374: f64, t39046: f64, t39215: f64, t39218: f64, t39221: f64, t39225: f64, t39229: f64, t39233: f64, t44014: f64, t44017: f64, t44020: f64, t44024: f64) -> (f64, f64, f64, f64) {
    let t44937 = 15.0_f64 / 8.0_f64 * t40282 * t12219;
    let t44940 = 135.0_f64 / 64.0_f64 * t3262 * t11189 * t42819;
    let t44942 = t3579 * t41298 / 2.0_f64;
    let t44953 = -0.2881692658299671676e-2_f64 * t39215 + 0.40992351065071538964e-3_f64 * t39218 + t39046 + 0.20496175532535769483e-3_f64 * t37362 + 0.1440846329149835838e-2_f64 * t39221 - 0.2881692658299671676e-2_f64 * t39225 + 0.40992351065071538964e-3_f64 * t39229 - 0.1440846329149835838e-2_f64 * t37366 - 0.7684513755465791136e-2_f64 * t39233 - 0.72042316457491791901e-3_f64 * t37370 + 0.72042316457491791901e-3_f64 * t37374 + t44014 + t44017 - t44020 - t44024;
    (t44937, t44940, t44942, t44953)
}
