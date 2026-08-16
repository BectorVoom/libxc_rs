//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1054/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1054(t26292: f64, t7289: f64, t25969: f64, t25975: f64, t26002: f64, t26010: f64, t26012: f64, t26021: f64, t2097: f64, t785: f64, t1358: f64, t2439: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26309 = 0.17135234354032049604e-1_f64 * t7289 * t26292;
    let t26310 = 0.54208002996571016773e-3_f64 * t25969;
    let t26312 = 0.22675591804667994221e-1_f64 * t25975;
    let t26321 = 35.0_f64 / 216.0_f64 * t26002;
    let t26324 = 0.10164000561857065645e-4_f64 * t26010;
    let t26325 = 0.30488190661738479625e-3_f64 * t26012;
    let t26328 = 0.18071592998981862717e-4_f64 * t26021;
    let t26358 = t785 * t2097;
    let t26359 = t26358 * t1358;
    let t26361 = 0.65049603595885220126e-3_f64 * t2439 * t26359;
    (t26309, t26310, t26312, t26321, t26324, t26325, t26328, t26358, t26359, t26361)
}
