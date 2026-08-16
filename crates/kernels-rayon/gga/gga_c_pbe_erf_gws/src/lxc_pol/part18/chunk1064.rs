//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1064/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1064(t2323: f64, t3875: f64, t3128: f64, t8833: f64, t2255: f64, t3757: f64, t9364: f64, t3257: f64, t3803: f64, t6345: f64, t11576: f64, t3131: f64, t3139: f64) -> (f64, f64, f64, f64, f64) {
    let t11944 = t2323 * t3875;
    let t11946 = t3128 * t8833;
    let t11947 = 7.0_f64 / 72.0_f64 * t11946;
    let t11949 = t2255 * t9364 * t3757;
    let t11953 = t3257 * t3803 * t6345;
    let t11957 = t3139 * t3131 * t11576;
    (t11944, t11947, t11949, t11953, t11957)
}
