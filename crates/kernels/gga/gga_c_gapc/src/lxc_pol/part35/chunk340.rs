//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 340/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk340<F: Float>(t519: F, t619: F, t1218: F, t1415: F, t1420: F, t1424: F, t1428: F, t1433: F, t1437: F, t1438: F, t1441: F, t1445: F, t1449: F, t1460: F, t1465: F, t1469: F, t1476: F, t434: F, t466: F, t473: F, t477: F, t518: F, t526: F, t569: F) -> F {
    let t1477 = t519 * t619;
    let t1480 = F::new(0.73256006569213709438e-5) * t1415 * t1420 - F::new(0.20855578275249024918e-2) * t526 * t1424 - F::new(0.20855578275249024918e-2) * t1428 * t569 + F::new(0.20855578275249024918e-2) * t434 * t1433 + F::new(0.6951859425083008306e-4) * t1437 * t1438 + F::new(0.6951859425083008306e-4) * t466 * t1441 + F::new(0.12360406057797588768e-3) * t473 * t1445 + F::new(0.1013812832824605378e-3) * t518 * t1449 + F::new(0.14784770478692161762e-4) * t1460 * t1465 - F::new(0.28840947468194373793e-3) * t1469 * t477 - F::new(0.1013812832824605378e-4) * t1476 * t1477 + t1218;
    t1480
}
