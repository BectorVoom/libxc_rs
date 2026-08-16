//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 971/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk971(t2359: f64, t2373: f64, t2388: f64, t2392: f64, t2498: f64, t2503: f64, t3040: f64, t3047: f64, t3077: f64, t3207: f64, t4415: f64, t6793: f64, t827: f64, t833: f64, t8584: f64, t8592: f64, t8598: f64, t8602: f64, t8606: f64, t8611: f64, t8616: f64, t8622: f64, t8624: f64) -> f64 {
    let t8628 = -t2388 * t3047 / 96.0_f64 - t2392 * t3047 / 96.0_f64 - t827 * t8584 / 48.0_f64 - t3040 * t2373 / 24.0_f64 - t827 * t8592 / 48.0_f64 - t2498 * t2373 / 24.0_f64 + t8598 + t6793 * t8602 / 8.0_f64 + t3077 * t8606 / 48.0_f64 - t2359 * t8611 / 96.0_f64 + t2388 * t2503 / 96.0_f64 + t8616 * t833 / 96.0_f64 + t2392 * t2503 / 96.0_f64 + t8622 + t3207 * t8624 / 8.0_f64 - 7.0_f64 / 72.0_f64 * t4415;
    t8628
}
