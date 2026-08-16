//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2122/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2122(t18437: f64, t7045: f64, t18348: f64, t1945: f64, t807: f64, t25266: f64, t6019: f64, t6024: f64, t93054: f64, t103297: f64, t99020: f64, t99022: f64, t99024: f64, t99027: f64, t99030: f64, t99034: f64, t99042: f64) -> f64 {
    let t106058 = t7045 * t18437;
    let t106061 = t807 * t1945 * t18348;
    let t106063 = t25266 * t6019;
    let t106065 = t93054 * t6024;
    let t106067 = 0.85748036236139473945e-2_f64 * t106058 + t99020 - t99022 - t99024 - t99027 + t99030 + t99034 - t103297 + t99042 + 0.57165357490759649296e-4_f64 * t106061 + 0.20007875121765877254e-2_f64 * t106063 - 0.40015750243531754507e-2_f64 * t106065;
    t106067
}
