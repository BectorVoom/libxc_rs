//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1328/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1328(t53806: f64, t14902: f64, t9270: f64, t14928: f64, t840: f64, t2409: f64, t2410: f64, t3066: f64, t36129: f64, t36200: f64, t36201: f64, t4097: f64, t4207: f64, t51719: f64, t51724: f64, t52473: f64, t53775: f64, t53804: f64, t53809: f64, t53811: f64, t53816: f64, t53832: f64, t53843: f64) -> f64 {
    let t55375 = 7.0_f64 / 12.0_f64 * t53806;
    let t55382 = 7.0_f64 / 72.0_f64 * t9270 * t14902;
    let t55385 = 7.0_f64 / 144.0_f64 * t840 * t14928;
    let t55392 = t36200 * t36201 * t4207 * t2410 / 4.0_f64 - t53775 / 24.0_f64 + t53804 / 384.0_f64 - t55375 + 7.0_f64 / 144.0_f64 * t51719 + t53809 / 8.0_f64 + t53811 / 4.0_f64 - 7.0_f64 / 72.0_f64 * t51724 - t53816 / 384.0_f64 - t55382 - t53832 / 2.0_f64 + t55385 - 7.0_f64 / 72.0_f64 * t52473 + t3066 * t2409 * t36129 * t4097 / 24.0_f64 - t53843 / 4.0_f64;
    t55392
}
