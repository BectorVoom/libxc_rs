//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1058/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1058<F: Float>(t2471: F, t848: F, t551: F, t8244: F, t35204: F, t35208: F, t35212: F, t35217: F, t35222: F, t35226: F, t35230: F, t35242: F, t35246: F, t37375: F, t39615: F, t39620: F, t39625: F, t39630: F, t39635: F, t739: F, t884: F) -> (F, F, F) {
    let t43065 = t2471 * t848;
    let t43080 = t8244 * t551;
    let t43083 = -F::cast_from(0.1702583995731913576e-4_f64) * t39615 + F::cast_from(0.212822999466489197e-4_f64) * t39620 + F::cast_from(0.59871208509319042821e-1_f64) * t884 * t43065 - F::cast_from(0.425645998932978394e-4_f64) * t39625 - F::cast_from(0.3405167991463827152e-4_f64) * t39630 + F::cast_from(0.10215503974391481456e-3_f64) * t39635 - F::cast_from(0.76845137554657911361e-2_f64) * t35204 + F::cast_from(0.18446557979282192534e-2_f64) * t35208 - F::cast_from(0.20496175532535769482e-3_f64) * t35212 + F::cast_from(0.1440846329149835838e-2_f64) * t35217 - F::cast_from(0.20496175532535769482e-3_f64) * t35222 + F::cast_from(0.12195059916630011325e-2_f64) * t35226 - F::cast_from(0.17347588262831798123e-3_f64) * t35230 + t37375 + F::cast_from(0.12195059916630011325e-2_f64) * t35242 - F::cast_from(0.17347588262831798123e-3_f64) * t35246 - F::cast_from(0.59871208509319042821e-1_f64) * t739 * t43080;
    (t43065, t43080, t43083)
}
