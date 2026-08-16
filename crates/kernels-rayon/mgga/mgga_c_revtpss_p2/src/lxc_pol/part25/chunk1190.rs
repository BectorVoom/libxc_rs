//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1190/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1190(t26028: f64, t3940: f64, t3926: f64, t7264: f64, t26003: f64, t26006: f64, t26007: f64, t26011: f64, t26013: f64, t26016: f64, t26018: f64, t26022: f64, t26025: f64) -> f64 {
    let t26029 = t26028 * t3940;
    let t26031 = t7264 * t3926;
    let t26033 = t26003 + t26006 - t26007 / 48.0_f64 - t26011 + t26013 + t26016 + t26018 / 16.0_f64 + t26022 + 0.40015750243531754508e-2_f64 * t26025 + 0.34299214494455789578e-2_f64 * t26029 - 0.42874018118069736972e-3_f64 * t26031;
    t26033
}
