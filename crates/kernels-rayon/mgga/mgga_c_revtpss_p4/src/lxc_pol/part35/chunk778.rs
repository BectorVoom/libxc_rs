//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 778/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk778(t12351: f64, t12295: f64, t3475: f64, t431: f64, t426: f64, t1159: f64, t3478: f64, t434: f64, t3519: f64, t444: f64, t439: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12352 = 0.36514074074074074075e0_f64 * t12351;
    let t12367 = 0.28842592592592592592e-1_f64 * t12295;
    let t12382 = 0.55403703703703703703e-1_f64 * t12295;
    let t12397 = 0.53272592592592592592e-1_f64 * t12295;
    let t12428 = 1.0_f64 / t3475 / t431;
    let t12429 = t426 * t12428;
    let t12459 = 0.16068111111111111111e1_f64 * t12295;
    let t12460 = 0.46308888888888888888e0_f64 * t12351;
    let t12469 = 1.0_f64 / t3475 / t1159;
    let t12470 = t426 * t12469;
    let t12472 = 1.0_f64 / t3478 / t434;
    let t12485 = 1.0_f64 / t3519 / t444;
    let t12486 = t439 * t12485;
    let t12542 = 0.93932222222222222223e0_f64 * t12295;
    (t12352, t12367, t12382, t12397, t12429, t12459, t12460, t12470, t12472, t12485, t12486, t12542)
}
