//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1212/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1212(t4083: f64, t6745: f64, t4110: f64, t6781: f64, t829: f64, t830: f64, t27047: f64, t3067: f64, t4097: f64, t814: f64, t20154: f64, t2376: f64, t4088: f64) -> (f64, f64, f64, f64) {
    let t52270 = t6745 * t4083;
    let t52274 = t6781 * t4110;
    let t52276 = t829 * t830 * t52274;
    let t52294 = t27047 * t3067 * t4097 * t814;
    let t52299 = t20154 * t2376 * t4088 * t814;
    (t52270, t52276, t52294, t52299)
}
