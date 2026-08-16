//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 920/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk920(t153: f64, t3373: f64, t542: f64, t3491: f64, t631: f64, t184: f64, t221: f64, t7041: f64, t3488: f64, t583: f64, t1791: f64, t3553: f64) -> (f64, f64, f64, f64, f64) {
    let t10283 = t153 * t542 * t3373;
    let t10287 = t3491 * t631;
    let t10288 = t10287 * t184;
    let t10290 = 4.0_f64 / 15.0_f64 * t10288 * t221;
    let t10291 = 32.0_f64 / 135.0_f64 * t7041;
    let t10293 = t3488 * t583;
    let t10294 = 4.0_f64 / 45.0_f64 * t10293;
    let t10295 = t1791 * t3553;
    (t10283, t10290, t10291, t10294, t10295)
}
