//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 765/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk765(t252: f64, t5385: f64, t1907: f64, t723: f64, t1697: f64, t212: f64, t22: f64, t1774: f64, t586: f64, t1651: f64, t1655: f64, t587: f64) -> (f64, f64, f64, f64, f64) {
    let t5387 = 8.0_f64 / 81.0_f64 * t252 * t5385;
    let t5388 = t1907 * t723;
    let t5399 = 1.0_f64 / t212 / t1697;
    let t5400 = t22 * t5399;
    let t5406 = t1774 * t586;
    let t5413 = t1651 * t1655;
    let t5414 = t587 * t5413;
    (t5387, t5388, t5400, t5406, t5414)
}
