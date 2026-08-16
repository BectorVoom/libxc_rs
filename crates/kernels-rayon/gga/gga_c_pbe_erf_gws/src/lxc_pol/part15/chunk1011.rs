//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1011/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1011(t1130: f64, t2074: f64, t339: f64, t8574: f64, t2178: f64, t2181: f64, t2183: f64, t2186: f64, t3154: f64, t3159: f64, t3162: f64, t340: f64, t6421: f64, t6424: f64, t6429: f64, t870: f64, t871: f64, t9050: f64, t9053: f64, t9056: f64, t9067: f64, t9070: f64) -> f64 {
    let t9073 = t1130 * t2074;
    let t9076 = t339 * t8574;
    let t9079 = -t339 * t340 * t9050 + 3.0_f64 * t1130 * t6421 + 6.0_f64 * t2178 * t3162 - 24.0_f64 * t2181 * t9070 - 12.0_f64 * t2181 * t9073 - 12.0_f64 * t2183 * t9056 + 3.0_f64 * t2186 * t3154 - 24.0_f64 * t3159 * t6424 + 60.0_f64 * t6429 * t9067 + 3.0_f64 * t870 * t9076 + 6.0_f64 * t871 * t9053;
    t9079
}
