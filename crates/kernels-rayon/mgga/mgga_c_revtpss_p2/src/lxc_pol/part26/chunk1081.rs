//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1081/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1081(t25231: f64, t25242: f64, t25253: f64, t25275: f64, t25283: f64, t25251: f64, t25256: f64, t25258: f64, t25263: f64, t25267: f64, t25271: f64, t25278: f64, t25280: f64) -> (f64, f64, f64) {
    let t26454 = 0.54208002996571016773e-3_f64 * t25231;
    let t26457 = 0.18071592998981862717e-4_f64 * t25242;
    let t26462 = 0.30488190661738479625e-3_f64 * t25253;
    let t26468 = 35.0_f64 / 216.0_f64 * t25275;
    let t26471 = 0.10164000561857065645e-4_f64 * t25283;
    let t26472 = -0.85748036236139473944e-3_f64 * t25251 + t26462 + 0.22866142996303859718e-3_f64 * t25256 - 0.85748036236139473944e-3_f64 * t25258 + 0.17149607247227894789e-2_f64 * t25263 + 0.80031500487063509014e-2_f64 * t25267 + 0.68598428988911579156e-2_f64 * t25271 + t26468 + 7.0_f64 / 36.0_f64 * t25278 - t25280 / 24.0_f64 - t26471;
    (t26454, t26457, t26472)
}
