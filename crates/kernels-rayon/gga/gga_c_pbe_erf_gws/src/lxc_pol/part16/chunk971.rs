//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 971/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk971(t3065: f64, t8605: f64, t2501: f64, t814: f64, t829: f64, t830: f64, t1114: f64, t6111: f64, t2367: f64, t3052: f64, t2395: f64, t2409: f64, t3189: f64) -> (f64, f64, f64, f64, f64) {
    let t8606 = t3065 * t8605;
    let t8611 = t829 * t830 * t2501 * t814;
    let t8616 = t1114 * t6111;
    let t8622 = 7.0_f64 / 72.0_f64 * t2367 * t3052;
    let t8624 = t2409 * t2395 * t3189;
    (t8606, t8611, t8616, t8622, t8624)
}
