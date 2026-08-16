//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 693/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk693(t599: f64, t922: f64, t142: f64, t7450: f64, t7388: f64, t7391: f64, t7394: f64, t7397: f64, t7398: f64, t7403: f64, t7406: f64, t7407: f64, t7409: f64, t7411: f64, t7416: f64, t7420: f64, t7424: f64, t7429: f64, t7435: f64, t7438: f64, t7442: f64, t7445: f64, t7449: f64) -> (f64, f64, f64) {
    let t7451 = t599 * t922;
    let t7452 = t142 * t7451;
    let t7453 = t7450 * t7452;
    let t7455 = -t7388 - t7391 + t7394 / 192.0_f64 + t7397 + t7398 / 48.0_f64 + t7403 / 32.0_f64 + t7406 - t7407 / 24.0_f64 - t7409 / 48.0_f64 - t7411 / 48.0_f64 - 0.31448092289604152068e-3_f64 * t7416 + 0.15724046144802076034e-3_f64 * t7420 - 0.10718504529517434243e-3_f64 * t7424 - 0.94344276868812456204e-3_f64 * t7429 - t7435 + t7438 / 24.0_f64 - t7442 - 0.22921875e-1_f64 * t7445 - t7449 - 0.4584375e-1_f64 * t7453;
    (t7451, t7452, t7455)
}
