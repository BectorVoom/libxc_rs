//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 447/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk447(t1724: f64, t650: f64, t186: f64, t211: f64, t202: f64, t631: f64, t184: f64) -> (f64, f64, f64, f64, f64) {
    let t1725 = t650 * t1724;
    let t1726 = t186 * t1725;
    let t1728 = 2.0_f64 / 15.0_f64 * t211 * t1726;
    let t1729 = t202 * t631;
    let t1730 = t1729 * t184;
    (t1725, t1726, t1728, t1729, t1730)
}
