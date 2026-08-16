//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 152/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk152(t467: f64, t473: f64, t198: f64, t330: f64, t410: f64, t438: f64, t440: f64, t259: f64) -> (f64, f64, f64) {
    let t475 = t467 * t473 + 1.0_f64;
    let t476 = f64::ln(t475);
    let t479 = t198 * t330 * t476 - t410 + t438 + t440;
    let t480 = t259 < t479;
    let t481 = piecewise3(t480, t479, t259);
    (t475, t481, t479)
}
