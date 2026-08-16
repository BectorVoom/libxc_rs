//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 831/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk831(t13187: f64, t2409: f64, t831: f64, t1115: f64, t11349: f64, t11368: f64, t12234: f64, t13142: f64, t13174: f64, t13184: f64, t2503: f64, t3047: f64, t3052: f64, t3207: f64, t3733: f64, t3913: f64, t833: f64, t8747: f64, t9815: f64, t9849: f64, t9912: f64, t9953: f64, t9956: f64, t9962: f64) -> (f64, f64) {
    let t13189 = t2409 * t831 * t13187;
    let t13201 = 7.0_f64 / 24.0_f64 * t9912 - 7.0_f64 / 24.0_f64 * t9953 + t13142 * t833 / 96.0_f64 - 7.0_f64 / 96.0_f64 * t9956 - 7.0_f64 / 96.0_f64 * t9962 + 35.0_f64 / 144.0_f64 * t8747 + t13174 * t833 / 96.0_f64 + t1115 * t12234 / 32.0_f64 + t3913 * t2503 / 32.0_f64 - 7.0_f64 / 96.0_f64 * t11349 - 3.0_f64 / 16.0_f64 * t3207 * t13184 + 3.0_f64 / 16.0_f64 * t3207 * t13189 + 7.0_f64 / 96.0_f64 * t11368 - t9815 * t3733 / 48.0_f64 - t9849 * t3733 / 48.0_f64 - t3913 * t3052 / 16.0_f64 - t3913 * t3047 / 32.0_f64;
    (t13189, t13201)
}
