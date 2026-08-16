//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 456/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk456(t1802: f64, t188: f64, t610: f64, t186: f64, t185: f64, t219: f64, t642: f64) -> (f64, f64, f64, f64, f64) {
    let t1803 = t188 * t1802;
    let t1804 = t610 * t610;
    let t1805 = t1803 * t1804;
    let t1806 = t186 * t1805;
    let t1808 = 4.0_f64 / 15.0_f64 * t185 * t1806;
    let t1809 = t642 * t219;
    (t1804, t1805, t1806, t1808, t1809)
}
