//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 510/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk510(t3443: f64, t598: f64, t186: f64, t185: f64, t2790: f64, t997: f64, t198: f64, t3345: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3444 = t598 * t3443;
    let t3445 = t186 * t3444;
    let t3447 = 2.0_f64 / 15.0_f64 * t185 * t3445;
    let t3449 = 8.0_f64 / 15.0_f64 * t2790 * t997;
    let t3450 = t198 * t3345;
    let t3451 = t186 * t3450;
    (t3444, t3445, t3447, t3449, t3450, t3451)
}
