//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 766/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk766(t12517: f64, t203: f64, t184: f64, t221: f64, t10293: f64, t10301: f64, t11191: f64, t12436: f64, t12438: f64, t12442: f64, t12446: f64, t12448: f64, t12450: f64, t12454: f64, t12488: f64, t4910: f64, t8405: f64, t8408: f64, t8414: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12518 = t203 * t12517;
    let t12519 = t12518 * t184;
    let t12521 = 2.0_f64 / 15.0_f64 * t12519 * t221;
    let t12524 = 4.0_f64 / 15.0_f64 * t10293;
    let t12525 = 16.0_f64 / 45.0_f64 * t10301;
    let t12526 = 0.32463124087094530131e0_f64 * t11191 + t12436 - t12438 - t12442 - t12446 + t12448 + t12450 + t12454 + t4910 + 4.0_f64 * t8405 + t12488 + t12521 + 0.21642082724729686754e0_f64 * t8408 + 0.64926248174189060262e0_f64 * t8414 + t12524 - t12525;
    (t12518, t12519, t12521, t12524, t12525, t12526)
}
