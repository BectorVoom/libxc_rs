//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 587/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk587(t338: f64, t353: f64, t4436: f64, t2200: f64, t329: f64, t340: f64, t847: f64, t2231: f64, t892: f64, t2366: f64, t2387: f64, t833: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4438 = t338 * t353 * t4436;
    let t4442 = t329 * t2200 * t340;
    let t4443 = t4442 * t847;
    let t4446 = t338 * t892 * t2231;
    let t4453 = t2387 * t2366;
    let t4454 = t4453 * t833;
    (t4438, t4442, t4443, t4446, t4453, t4454)
}
