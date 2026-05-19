//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1139/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1139<F: Float>(t10257: F, t1356: F, t1550: F, t38047: F, t43878: F, t43891: F, t43892: F, t43911: F, t47644: F, t47646: F, t47653: F, t47663: F, t47667: F, t47669: F, t47676: F, t47680: F, t49294: F, t49311: F, t49323: F, t49336: F, t49351: F, t49365: F, t49380: F, t49398: F, t49424: F, t49452: F, t49475: F, t49501: F, t49507: F, t49510: F, t49533: F, t49557: F, t49567: F, t49591: F, t49606: F, t4965: F, t72: F, t739: F, t82: F) -> F {
    let t49626 = F::cast_from(0.2553875993597870364e-4_f64) * t47644 - t43878 - F::cast_from(0.11918087970123395032e-3_f64) * t47646 + F::cast_from(0.10215503974391481456e-3_f64) * t47653 + t72 * t82 * (t49294 + t49311 + t49323 + t49336 + t49351 + t49365 + t49380 + t49398 + t49424 + t49452 + t49475 + t49501 + t49533 + t49567 + t49591 + t49606) - F::cast_from(0.11974241701863808564e0_f64) * t1550 * t49510 + F::cast_from(0.79828278012425390428e-1_f64) * t1356 * t49507 - F::cast_from(0.11918087970123395032e-3_f64) * t47663 - F::cast_from(0.85129199786595678799e-5_f64) * t47667 - t38047 + F::cast_from(0.39914139006212695214e-1_f64) * t4965 * t10257 + t43891 + t43892 + F::cast_from(0.11918087970123395032e-3_f64) * t47669 + t43911 + F::cast_from(0.30487649791575028312e-3_f64) * t47676 + F::cast_from(0.30487649791575028312e-3_f64) * t47680 - F::cast_from(0.11974241701863808564e0_f64) * t739 * t49557;
    t49626
}
