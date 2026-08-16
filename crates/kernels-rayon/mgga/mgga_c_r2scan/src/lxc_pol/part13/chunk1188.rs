//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1188/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1188(t1048: f64, t39383: f64, t39418: f64, t39450: f64, t39478: f64, t39504: f64, t39537: f64, t39567: f64, t39590: f64, t39624: f64, t39652: f64, t39681: f64, t39710: f64, t39735: f64, t39754: f64, t39784: f64, t39809: f64, t39837: f64, t39871: f64, t39896: f64, t39932: f64, t39956: f64, t39989: f64, t40023: f64, t40055: f64, t40088: f64, t40111: f64, t40144: f64, t40167: f64, t40193: f64, t40230: f64, t40247: f64, t40262: f64, t499: f64, t797: f64) -> f64 {
    let t40271 = t1048 * t499 * (t39710 + t40111 + t40144 + t40247 + t39681 + t39956 + t39754 + t40262 + t39989 + t40167 + t39837 + t39784 + t39871 + t39652 + t40230 + t40088 + t39896 + t39504 + t39383 + t39450 + t40023 + t39567 + t40055 + t39590 + t39478 + t40193 + t39624 + t39418 + t39932 + t39809 + t39537 + t39735) * t797 / 4.0_f64;
    t40271
}
