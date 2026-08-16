//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2296/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2296(t24574: f64, t29554: f64, t1240: f64, t6267: f64, t2122: f64, t29817: f64, t1184: f64, t6145: f64, t1186: f64, t1409: f64, t1761: f64, t19234: f64, t19249: f64, t24589: f64, t24601: f64, t24602: f64, t27406: f64, t27416: f64, t27437: f64, t27799: f64, t27800: f64, t29690: f64, t29808: f64, t5088: f64, t7283: f64, t7356: f64, t85807: f64, t86415: f64, t94458: f64, t94535: f64, t95836: f64) -> (f64, f64, f64) {
    let t103304 = t24574 * t29554;
    let t103314 = t1240 * t6267;
    let t103315 = t2122 * t103314;
    let t103332 = t24574 * t29817;
    let t103337 = t6145 * t1184;
    let t103341 = -0.54831135561607547883e-2_f64 * t103304 + t94535 + 0.43864908449286038306e-1_f64 * t27406 * t27416 - 2.0_f64 * t95836 * t1761 + 2.0_f64 * t19249 * t7356 + 4.0_f64 * t19234 * t7356 + 0.82246703342411321825e-2_f64 * t7283 * t1186 * t103315 + 0.43864908449286038306e-1_f64 * t27406 * t27800 + 0.54831135561607547883e-2_f64 * t24589 * t94458 * t27437 + 0.54831135561607547884e-2_f64 * t24589 * t86415 * t29808 + 0.54831135561607547884e-2_f64 * t24589 * t24601 * t24602 * t1409 * t5088 - 0.18277045187202515961e-2_f64 * t103332 + 0.36554090374405031923e-2_f64 * t7283 * t85807 * t29690 - 0.82246703342411321825e-2_f64 * t7283 * t103337 * t27799;
    (t103314, t103337, t103341)
}
