//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1112/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1112(t13953: f64, t3966: f64, t2367: f64, t4002: f64, t3979: f64, t3997: f64, t1176: f64, t903: f64, t923: f64) -> (f64, f64, f64, f64) {
    let t13954 = t13953 * t3966;
    let t13962 = t2367 * t4002;
    let t13964 = t3979 * t3997;
    let t13972 = t1176 * t923 * t903;
    (t13954, t13962, t13964, t13972)
}
