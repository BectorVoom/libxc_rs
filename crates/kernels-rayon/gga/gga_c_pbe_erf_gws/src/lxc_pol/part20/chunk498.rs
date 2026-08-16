//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 498/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk498(t2118: f64, t825: f64, t814: f64, t830: f64, t831: f64, t829: f64, t328: f64, t837: f64) -> (f64, f64, f64) {
    let t2358 = t2118 * t825;
    let t2361 = t830 * t831 * t814;
    let t2362 = t829 * t2361;
    let t2365 = t328 * t837;
    (t2358, t2362, t2365)
}
