//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 596/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk596(t2705: f64, t625: f64, t2704: f64, t1041: f64, t401: f64, t1714: f64, t2679: f64, t2673: f64, t657: f64, t1472: f64, t21: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2706 = t625 * t2705;
    let t2707 = t2704 * t2706;
    let t2710 = t401 * t1041;
    let t2712 = t1714 * t2679;
    let t2715 = t657 * t2673;
    let t2718 = t21 * t1472;
    (t2706, t2707, t2710, t2712, t2715, t2718)
}
