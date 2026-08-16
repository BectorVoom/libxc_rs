//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1078/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1078(t12098: f64, t898: f64, t338: f64, t353: f64, t3067: f64, t3721: f64, t829: f64, t830: f64, t1118: f64, t3200: f64, t1144: f64, t3097: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12099 = t898 * t12098;
    let t12101 = t338 * t353 * t12099;
    let t12109 = t3067 * t3721;
    let t12111 = t829 * t830 * t12109;
    let t12121 = t338 * t3200 * t1118;
    let t12125 = t338 * t1144 * t3097;
    (t12099, t12101, t12109, t12111, t12121, t12125)
}
