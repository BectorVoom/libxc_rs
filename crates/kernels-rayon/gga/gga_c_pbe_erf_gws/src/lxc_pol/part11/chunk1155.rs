//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1155/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1155(t31168: f64, t3411: f64, t247: f64, t251: f64, t25395: f64, t256: f64, t48313: f64, t48315: f64, t48316: f64, t48318: f64, t48320: f64, t48321: f64, t48330: f64, t48359: f64, t48363: f64, t48367: f64) -> (f64, f64) {
    let t48369 = 32.0_f64 / 15.0_f64 * t31168 * t3411;
    let t48370 = -t48313 + t48315 + t48316 - 32.0_f64 / 405.0_f64 * t25395 + t48318 + t48320 + t48321 * t247 * t251 * t256 / 3.0_f64 + t48330 + t48359 - t48363 + t48367 + t48369;
    (t48369, t48370)
}
