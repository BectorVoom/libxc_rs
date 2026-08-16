//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1185/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1185(t14106: f64, t2376: f64, t829: f64, t830: f64, t13793: f64, t50943: f64, t13803: f64, t13808: f64, t1192: f64, t20154: f64, t810: f64, t814: f64) -> (f64, f64, f64, f64) {
    let t50965 = t2376 * t14106;
    let t50967 = t829 * t830 * t50965;
    let t50970 = t50943 * t13793;
    let t50972 = t13808 * t13803;
    let t50977 = t20154 * t2376 * t1192 * t814 * t810;
    (t50967, t50970, t50972, t50977)
}
