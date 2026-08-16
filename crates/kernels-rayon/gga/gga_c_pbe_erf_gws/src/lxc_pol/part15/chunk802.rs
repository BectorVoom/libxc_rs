//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 802/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk802(t6161: f64, t829: f64, t830: f64, t831: f64, t2420: f64, t840: f64, t2355: f64, t2156: f64, t5: f64, t343: f64, t337: f64, t2121: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6164 = t829 * t830 * t831 * t6161;
    let t6173 = t840 * t2420;
    let t6175 = t840 * t2355;
    let t6177 = t5 * t2156;
    let t6178 = t6177 * t343;
    let t6179 = t337 * t6178;
    let t6180 = t2121 * t6179;
    (t6164, t6173, t6175, t6177, t6178, t6180)
}
