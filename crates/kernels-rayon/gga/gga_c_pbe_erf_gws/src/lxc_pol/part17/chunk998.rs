//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 998/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk998(t6333: f64, t3128: f64, t6258: f64, t8923: f64, t8925: f64, t8927: f64, t8930: f64, t8932: f64, t8936: f64, t8938: f64, t8943: f64, t8948: f64, t8951: f64) -> (f64, f64, f64) {
    let t8952 = 7.0_f64 / 72.0_f64 * t6333;
    let t8954 = t3128 * t6258 / 48.0_f64;
    let t8955 = t8923 - t8925 - t8927 - t8930 + t8932 + t8936 - t8938 - t8943 + t8948 - t8951 + t8952 - t8954;
    (t8952, t8954, t8955)
}
