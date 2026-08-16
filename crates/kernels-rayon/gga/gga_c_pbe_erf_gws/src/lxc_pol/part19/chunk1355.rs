//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1355/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1355(t15559: f64, t4414: f64, t11396: f64, t12204: f64, t14185: f64, t2408: f64, t3066: f64, t54896: f64, t54902: f64, t54904: f64, t55297: f64, t56113: f64, t56116: f64, t56119: f64, t56124: f64, t56126: f64, t56129: f64, t56142: f64, t56147: f64, t56153: f64, t9283: f64) -> f64 {
    let t57984 = t4414 * t15559;
    let t57994 = -t56113 / 24.0_f64 + t56116 / 24.0_f64 + t56119 / 8.0_f64 + t56124 / 48.0_f64 - 7.0_f64 / 576.0_f64 * t56126 - t56129 / 384.0_f64 - 7.0_f64 / 144.0_f64 * t56142 - 7.0_f64 / 144.0_f64 * t56147 - t54896 + t56153 / 24.0_f64 + 7.0_f64 / 36.0_f64 * t57984 - t2408 * t9283 * t14185 * t11396 / 24.0_f64 + t3066 * t9283 * t55297 * t12204 / 4.0_f64 - t54902 + t54904;
    t57994
}
