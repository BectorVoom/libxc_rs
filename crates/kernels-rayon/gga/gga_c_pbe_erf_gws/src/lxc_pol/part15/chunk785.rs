//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 785/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk785(t496: f64, t5818: f64, t505: f64, t96: f64, t1235: f64, t125: f64, t128: f64, t2: f64, t39: f64, t1570: f64, t513: f64, t1576: f64, t510: f64) -> (f64, f64, f64, f64, f64) {
    let t5819 = t496 * t5818;
    let t5825 = 1.0_f64 / t505 / t96;
    let t5832 = t125 * t1235;
    let t5833 = t128 * t2;
    let t5836 = 0.32645333333333333334e0_f64 * t5832 * t5833 * t39;
    let t5844 = t1570 * t513;
    let t5847 = t510 * t1576;
    (t5819, t5825, t5836, t5844, t5847)
}
