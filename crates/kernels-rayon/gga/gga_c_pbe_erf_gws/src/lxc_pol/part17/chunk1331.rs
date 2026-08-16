//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1331/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1331(t4142: f64, t51529: f64, t13953: f64, t14648: f64, t51877: f64, t14404: f64, t14710: f64, t19895: f64, t22142: f64, t2220: f64, t29751: f64, t3189: f64, t3207: f64, t335: f64, t338: f64, t353: f64, t4002: f64, t4053: f64, t4183: f64, t51081: f64, t51087: f64, t51864: f64, t51870: f64, t51881: f64, t51896: f64, t54018: f64, t54041: f64, t54074: f64, t54104: f64, t54132: f64, t54156: f64, t54181: f64, t54211: f64, t54235: f64, t54263: f64, t54291: f64, t54312: f64, t54337: f64, t54364: f64, t54388: f64, t54413: f64, t8793: f64, t898: f64, t9283: f64) -> f64 {
    let t54427 = t51529 * t4142;
    let t54429 = t13953 * t14648;
    let t54430 = 7.0_f64 / 144.0_f64 * t54429;
    let t54435 = 35.0_f64 / 216.0_f64 * t51877;
    let t54449 = -7.0_f64 / 72.0_f64 * t51864 - t335 * t338 * t353 * t898 * (t54018 + t54041 + t54074 + t54104 + t54132 + t54156 + t54181 + t54211 + t54235 + t54263 + t54291 + t54312 + t54337 + t54364 + t54388 + t54413) / 96.0_f64 - t335 * t338 * t2220 * t4183 / 96.0_f64 - 119.0_f64 / 3456.0_f64 * t54427 + t54430 - t51870 + t8793 * t51081 / 24.0_f64 + t8793 * t51087 / 24.0_f64 + t54435 + 7.0_f64 / 144.0_f64 * t51881 + t19895 * t14404 / 48.0_f64 - t22142 * t4002 / 96.0_f64 - 7.0_f64 / 2304.0_f64 * t51896 - t3207 * t29751 * t14710 / 8.0_f64 - t3207 * t9283 * t4053 * t3189 / 8.0_f64;
    t54449
}
