//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1071/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1071(t19301: f64, t496: f64, t19259: f64, t19264: f64, t19266: f64, t19270: f64, t19274: f64, t19279: f64, t19282: f64, t19286: f64, t19290: f64, t19294: f64, t19296: f64, t19299: f64) -> f64 {
    let t19302 = t496 * t19301;
    let t19304 = -0.587616e1_f64 * t19259 - t19264 + 8.0_f64 * t19266 + 30.0_f64 * t496 * t19270 + 9.0_f64 / 2.0_f64 * t496 * t19274 + 0.2350464e2_f64 * t19279 + t19282 + t19286 + t19290 + t19294 + 56.0_f64 / 27.0_f64 * t19296 - 4.0_f64 / 3.0_f64 * t19299 + 2.0_f64 / 3.0_f64 * t19302;
    t19304
}
