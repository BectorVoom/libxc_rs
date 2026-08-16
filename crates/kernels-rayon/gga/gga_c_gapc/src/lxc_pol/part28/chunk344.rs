//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 344/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk344(t519: f64, t619: f64, t1218: f64, t1415: f64, t1420: f64, t1424: f64, t1428: f64, t1433: f64, t1437: f64, t1438: f64, t1441: f64, t1445: f64, t1449: f64, t1460: f64, t1465: f64, t1469: f64, t1476: f64, t434: f64, t466: f64, t473: f64, t477: f64, t518: f64, t526: f64, t569: f64) -> f64 {
    let t1477 = t519 * t619;
    let t1480 = 0.73256006569213709438e-5_f64 * t1415 * t1420 - 0.20855578275249024918e-2_f64 * t526 * t1424 - 0.20855578275249024918e-2_f64 * t1428 * t569 + 0.20855578275249024918e-2_f64 * t434 * t1433 + 0.6951859425083008306e-4_f64 * t1437 * t1438 + 0.6951859425083008306e-4_f64 * t466 * t1441 + 0.12360406057797588768e-3_f64 * t473 * t1445 + 0.1013812832824605378e-3_f64 * t518 * t1449 + 0.14784770478692161762e-4_f64 * t1460 * t1465 - 0.28840947468194373793e-3_f64 * t1469 * t477 - 0.1013812832824605378e-4_f64 * t1476 * t1477 + t1218;
    t1480
}
