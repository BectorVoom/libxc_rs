//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 813/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk813(t378: f64, t6729: f64, t6182: f64, t6186: f64, t6190: f64, t6219: f64, t6224: f64, t6230: f64, t6246: f64, t6251: f64, t6255: f64, t6260: f64, t6273: f64, t6321: f64, t6324: f64) -> (f64, f64) {
    let t6731 = 455.0_f64 / 1296.0_f64 * t6729 * t378;
    let t6732 = -t6182 + t6186 - t6190 - t6219 + t6224 - t6230 - t6246 + t6251 - t6255 - t6260 + t6273 - t6321 - t6324;
    (t6731, t6732)
}
