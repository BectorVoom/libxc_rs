//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1230/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1230(t49416: f64, t49417: f64, t49419: f64, t49420: f64, t49424: f64, t49427: f64, t49429: f64, t49433: f64, t3776: f64, t3373: f64, t1076: f64, t11318: f64, t12381: f64, t13164: f64, t13167: f64, t2107: f64, t21091: f64, t22688: f64, t3030: f64, t323: f64, t35109: f64, t44405: f64, t48520: f64, t6096: f64, t818: f64, t9150: f64) -> (f64, f64) {
    let t49436 = t49416 + t49417 + t49419 + t49420 + t49424 + t49427 + t49429 + t49433;
    let t49450 = t3776 * t3776;
    let t49456 = t3373 * t3373;
    let t49463 = 8.0_f64 * t1076 * t12381 * t2107 - 36.0_f64 * t3373 * t3776 * t6096 - 4.0_f64 * t1076 * t44405 - 6.0_f64 * t11318 * t3373 - 4.0_f64 * t12381 * t3030 - 24.0_f64 * t13164 * t22688 + 24.0_f64 * t13167 * t9150 + 6.0_f64 * t2107 * t49456 + 24.0_f64 * t21091 * t49450 + t323 * t49436 + 12.0_f64 * t35109 * t3776 - t48520 * t818;
    (t49436, t49463)
}
