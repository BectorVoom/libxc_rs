//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 582/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk582(t4395: f64, t825: f64, t2382: f64, t2352: f64, t2376: f64, t829: f64, t830: f64, t2358: f64, t2387: f64, t2083: f64, t745: f64) -> (f64, f64, f64, f64, f64) {
    let t4396 = t4395 * t825;
    let t4397 = t2382 * t4396;
    let t4400 = t2376 * t2352;
    let t4402 = t829 * t830 * t4400;
    let t4405 = t2387 * t2358;
    let t4408 = t2083 * t745;
    (t4396, t4397, t4402, t4405, t4408)
}
