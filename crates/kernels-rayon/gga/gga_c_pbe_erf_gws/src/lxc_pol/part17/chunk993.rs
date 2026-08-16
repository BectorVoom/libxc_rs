//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 993/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk993(t2407: f64, t8896: f64, t6672: f64, t2142: f64, t3120: f64, t332: f64, t6238: f64, t863: f64, t2156: f64, t6241: f64, t3131: f64, t3139: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8897 = t2407 * t8896;
    let t8899 = t6672 * t8897 / 24.0_f64;
    let t8901 = 7.0_f64 / 144.0_f64 * t3120 * t2142;
    let t8903 = t863 * t6238 * t332;
    let t8904 = t6241 * t2156;
    let t8906 = t3139 * t3131 * t8904;
    (t8897, t8899, t8901, t8903, t8904, t8906)
}
