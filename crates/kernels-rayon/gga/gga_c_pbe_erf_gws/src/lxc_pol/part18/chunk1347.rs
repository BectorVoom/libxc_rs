//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1347/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1347(t11585: f64, t4028: f64, t11693: f64, t51274: f64, t14058: f64, t3875: f64, t36666: f64, t850: f64, t14093: f64, t51412: f64, t51415: f64, t54330: f64, t54345: f64, t57171: f64, t57174: f64, t57176: f64, t57179: f64, t57182: f64) -> f64 {
    let t57184 = t4028 * t11585;
    let t57186 = t51274 * t11693;
    let t57188 = t14058 * t3875;
    let t57190 = t850 * t36666;
    let t57191 = t57190 * t14093;
    let t57193 = -t57171 / 768.0_f64 - t57174 / 96.0_f64 + 7.0_f64 / 1152.0_f64 * t57176 + t57179 / 16.0_f64 - t54330 - 35.0_f64 / 216.0_f64 * t51412 - t51415 - 7.0_f64 / 384.0_f64 * t57182 - t57184 / 16.0_f64 - t57186 / 16.0_f64 - 35.0_f64 / 576.0_f64 * t57188 - t57191 / 96.0_f64 - t54345;
    t57193
}
