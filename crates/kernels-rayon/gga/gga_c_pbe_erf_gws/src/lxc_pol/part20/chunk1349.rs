//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1349/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1349(t12080: f64, t14101: f64, t54355: f64, t54378: f64, t55607: f64, t55609: f64, t55623: f64, t57195: f64, t57197: f64, t57199: f64, t57201: f64, t57204: f64, t57206: f64, t57208: f64) -> f64 {
    let t57210 = t14101 * t12080;
    let t57212 = -t55607 + t54355 - t55609 + t54378 - t57195 / 384.0_f64 - t57197 / 192.0_f64 - t57199 / 192.0_f64 - t55623 + 7.0_f64 / 288.0_f64 * t57201 + t57204 / 24.0_f64 - 7.0_f64 / 288.0_f64 * t57206 + t57208 / 24.0_f64 + t57210 / 16.0_f64;
    t57212
}
