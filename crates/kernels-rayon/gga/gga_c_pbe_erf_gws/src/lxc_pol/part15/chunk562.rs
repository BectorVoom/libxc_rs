//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 562/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk562(t329: f64, t332: f64, t369: f64, t2182: f64, t376: f64, t353: f64, t338: f64, t2169: f64) -> (f64, f64, f64, f64, f64) {
    let t2401 = t329 * t332 * t369;
    let t2402 = t376 * t2182;
    let t2403 = t353 * t2402;
    let t2404 = t338 * t2403;
    let t2407 = t332 * t2169;
    (t2401, t2402, t2403, t2404, t2407)
}
