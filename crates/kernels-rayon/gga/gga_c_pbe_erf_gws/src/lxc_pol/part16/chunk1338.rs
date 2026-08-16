//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1338/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1338(t51285: f64, t51293: f64, t51302: f64, t51315: f64, t51330: f64, t51332: f64, t54215: f64, t54217: f64, t54219: f64, t54224: f64, t54226: f64, t54231: f64) -> f64 {
    let t55546 = -t54215 / 48.0_f64 + t54217 / 192.0_f64 + t54219 / 384.0_f64 - 7.0_f64 / 576.0_f64 * t51285 + 7.0_f64 / 36.0_f64 * t51293 - 7.0_f64 / 192.0_f64 * t51302 - t54224 / 96.0_f64 + 5.0_f64 / 192.0_f64 * t54226 - 7.0_f64 / 288.0_f64 * t51315 - t54231 / 24.0_f64 + 7.0_f64 / 144.0_f64 * t51330 - 7.0_f64 / 576.0_f64 * t51332;
    t55546
}
