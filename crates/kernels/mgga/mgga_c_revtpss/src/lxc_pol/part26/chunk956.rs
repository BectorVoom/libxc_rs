//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 956/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk956<F: Float>(t25933: F, t26304: F, t26292: F, t7289: F, t25969: F, t25975: F, t26002: F, t26010: F, t26012: F, t26021: F, t26005: F, t26007: F, t26015: F, t26018: F, t26025: F, t26029: F, t26031: F) -> (F, F, F, F, F) {
    let t26305 = t26304 * t25933;
    let t26309 = 0.17135234354032049604e-1 * t7289 * t26292;
    let t26310 = 0.54208002996571016773e-3 * t25969;
    let t26312 = 0.22675591804667994221e-1 * t25975;
    let t26321 = 35.0 / 216.0 * t26002;
    let t26324 = 0.10164000561857065645e-4 * t26010;
    let t26325 = 0.30488190661738479625e-3 * t26012;
    let t26328 = 0.18071592998981862717e-4 * t26021;
    let t26332 = t26321 + 7.0 / 36.0 * t26005 - t26007 / 24.0 - t26324 + t26325 + 0.22866142996303859718e-3 * t26015 + t26018 / 8.0 + t26328 + 0.80031500487063509014e-2 * t26025 + 0.68598428988911579156e-2 * t26029 - 0.85748036236139473944e-3 * t26031;
    (t26305, t26309, t26310, t26312, t26332)
}
