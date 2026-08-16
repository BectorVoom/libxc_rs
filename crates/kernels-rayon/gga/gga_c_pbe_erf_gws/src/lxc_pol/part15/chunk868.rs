//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 868/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk868(t1809: f64, t7448: f64, t1620: f64, t1022: f64, t1642: f64, t1413: f64, t2677: f64, t7324: f64, t7416: f64, t7417: f64, t7418: f64, t7419: f64, t7420: f64, t7422: f64, t7423: f64, t7424: f64, t7427: f64, t7431: f64, t7434: f64, t7438: f64, t7442: f64, t7447: f64) -> (f64, f64, f64) {
    let t7449 = t1809 * t7448;
    let t7451 = 8.0_f64 / 45.0_f64 * t1620 * t7449;
    let t7452 = t1022 * t1642;
    let t7453 = t7452 * t1413;
    let t7454 = t2677 * t7453;
    let t7456 = 8.0_f64 / 27.0_f64 * t1620 * t7454;
    let t7457 = -t7324 - t7416 - t7417 - t7418 + t7419 + t7420 - t7422 - t7423 - t7424 + t7427 + t7431 + t7434 + t7438 - t7442 - t7447 + t7451 + t7456;
    (t7451, t7456, t7457)
}
