//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 214/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk214(t550: f64, t551: f64, t553: f64, t163: f64, t169: f64, t234: f64, t299: f64, t172: f64, t181: f64, t184: f64) -> (f64, f64, f64, f64) {
    let t555 = 0.19753890328909480882e-2_f64 * t550 * t551 * t553;
    let t559 = 0.89806755076909568204e-2_f64 * t169 * t299 * t234 * t163;
    let t560 = t172 * t181;
    let t561 = t560 * t184;
    (t555, t559, t560, t561)
}
