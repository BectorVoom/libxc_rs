//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1131/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1131(t13792: f64, t14469: f64, t338: f64, t4183: f64, t892: f64, t1115: f64, t13772: f64, t13809: f64, t13939: f64, t14437: f64, t14444: f64, t14448: f64, t14452: f64, t14457: f64, t14460: f64, t14464: f64, t14467: f64, t2408: f64, t3066: f64, t335: f64, t827: f64) -> (f64, f64) {
    let t14470 = t13792 * t14469;
    let t14473 = t338 * t892 * t4183;
    let t14477 = -t1115 * t13939 / 96.0_f64 - t827 * t14437 / 96.0_f64 - t1115 * t13772 / 96.0_f64 + t14444 / 3072.0_f64 + t2408 * t14448 / 48.0_f64 + t3066 * t14452 / 48.0_f64 + t14457 / 768.0_f64 + t2408 * t14460 / 48.0_f64 - t14464 / 48.0_f64 - t14467 / 48.0_f64 - t14470 / 48.0_f64 - t335 * t14473 / 96.0_f64 - 7.0_f64 / 2304.0_f64 * t13809;
    (t14473, t14477)
}
