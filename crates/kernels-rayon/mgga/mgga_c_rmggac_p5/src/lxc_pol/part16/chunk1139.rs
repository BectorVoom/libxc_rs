//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1139/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1139(t10257: f64, t1356: f64, t1550: f64, t38047: f64, t43878: f64, t43891: f64, t43892: f64, t43911: f64, t47644: f64, t47646: f64, t47653: f64, t47663: f64, t47667: f64, t47669: f64, t47676: f64, t47680: f64, t49294: f64, t49311: f64, t49323: f64, t49336: f64, t49351: f64, t49365: f64, t49380: f64, t49398: f64, t49424: f64, t49452: f64, t49475: f64, t49501: f64, t49507: f64, t49510: f64, t49533: f64, t49557: f64, t49567: f64, t49591: f64, t49606: f64, t4965: f64, t72: f64, t739: f64, t82: f64) -> f64 {
    let t49626 = 0.2553875993597870364e-4_f64 * t47644 - t43878 - 0.11918087970123395032e-3_f64 * t47646 + 0.10215503974391481456e-3_f64 * t47653 + t72 * t82 * (t49294 + t49311 + t49323 + t49336 + t49351 + t49365 + t49380 + t49398 + t49424 + t49452 + t49475 + t49501 + t49533 + t49567 + t49591 + t49606) - 0.11974241701863808564e0_f64 * t1550 * t49510 + 0.79828278012425390428e-1_f64 * t1356 * t49507 - 0.11918087970123395032e-3_f64 * t47663 - 0.85129199786595678799e-5_f64 * t47667 - t38047 + 0.39914139006212695214e-1_f64 * t4965 * t10257 + t43891 + t43892 + 0.11918087970123395032e-3_f64 * t47669 + t43911 + 0.30487649791575028312e-3_f64 * t47676 + 0.30487649791575028312e-3_f64 * t47680 - 0.11974241701863808564e0_f64 * t739 * t49557;
    t49626
}
