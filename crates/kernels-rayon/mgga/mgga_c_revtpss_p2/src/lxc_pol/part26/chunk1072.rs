//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1072/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1072(t25933: f64, t26304: f64, t26292: f64, t7289: f64, t25969: f64, t25975: f64, t26002: f64, t26010: f64, t26012: f64, t26021: f64, t26005: f64, t26007: f64, t26015: f64, t26018: f64, t26025: f64, t26029: f64, t26031: f64) -> (f64, f64, f64, f64, f64) {
    let t26305 = t26304 * t25933;
    let t26309 = 0.17135234354032049604e-1_f64 * t7289 * t26292;
    let t26310 = 0.54208002996571016773e-3_f64 * t25969;
    let t26312 = 0.22675591804667994221e-1_f64 * t25975;
    let t26321 = 35.0_f64 / 216.0_f64 * t26002;
    let t26324 = 0.10164000561857065645e-4_f64 * t26010;
    let t26325 = 0.30488190661738479625e-3_f64 * t26012;
    let t26328 = 0.18071592998981862717e-4_f64 * t26021;
    let t26332 = t26321 + 7.0_f64 / 36.0_f64 * t26005 - t26007 / 24.0_f64 - t26324 + t26325 + 0.22866142996303859718e-3_f64 * t26015 + t26018 / 8.0_f64 + t26328 + 0.80031500487063509014e-2_f64 * t26025 + 0.68598428988911579156e-2_f64 * t26029 - 0.85748036236139473944e-3_f64 * t26031;
    (t26305, t26309, t26310, t26312, t26332)
}
