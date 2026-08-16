//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1026/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1026(t12323: f64, t331: f64, t551: f64, t553: f64, t145: f64, t164: f64, t12891: f64, t547: f64, t12882: f64, t163: f64, t169: f64, t299: f64) -> (f64, f64, f64, f64, f64) {
    let t42244 = t331 * t12323 * t551 * t553;
    let t42251 = t145 * t12323;
    let t42252 = t42251 * t164;
    let t42265 = t12891 * t547;
    let t42272 = t169 * t299 * t12882 * t163;
    (t42244, t42251, t42252, t42265, t42272)
}
