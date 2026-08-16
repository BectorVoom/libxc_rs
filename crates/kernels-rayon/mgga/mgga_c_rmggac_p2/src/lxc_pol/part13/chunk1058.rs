//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1058/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1058(t2471: f64, t848: f64, t551: f64, t8244: f64, t35204: f64, t35208: f64, t35212: f64, t35217: f64, t35222: f64, t35226: f64, t35230: f64, t35242: f64, t35246: f64, t37375: f64, t39615: f64, t39620: f64, t39625: f64, t39630: f64, t39635: f64, t739: f64, t884: f64) -> (f64, f64, f64) {
    let t43065 = t2471 * t848;
    let t43080 = t8244 * t551;
    let t43083 = -0.1702583995731913576e-4_f64 * t39615 + 0.212822999466489197e-4_f64 * t39620 + 0.59871208509319042821e-1_f64 * t884 * t43065 - 0.425645998932978394e-4_f64 * t39625 - 0.3405167991463827152e-4_f64 * t39630 + 0.10215503974391481456e-3_f64 * t39635 - 0.76845137554657911361e-2_f64 * t35204 + 0.18446557979282192534e-2_f64 * t35208 - 0.20496175532535769482e-3_f64 * t35212 + 0.1440846329149835838e-2_f64 * t35217 - 0.20496175532535769482e-3_f64 * t35222 + 0.12195059916630011325e-2_f64 * t35226 - 0.17347588262831798123e-3_f64 * t35230 + t37375 + 0.12195059916630011325e-2_f64 * t35242 - 0.17347588262831798123e-3_f64 * t35246 - 0.59871208509319042821e-1_f64 * t739 * t43080;
    (t43065, t43080, t43083)
}
