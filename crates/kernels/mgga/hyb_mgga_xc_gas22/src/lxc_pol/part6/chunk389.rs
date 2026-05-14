//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 389/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk389<F: Float>(t1483: F, t1486: F, t209: F, t328: F, t407: F, t430: F, t194: F, t418: F, t196: F, t414: F, t423: F, t211: F, t429: F, t428: F, t1297: F, t1304: F, t1311: F, t1315: F, t1454: F, t1455: F, t1459: F, t1464: F, t1469: F, t1474: F, t1475: F, t1479: F, t1480: F, t198: F, t398: F, t401: F, t405: F, t408: F, t415: F, t431: F, tau1: F) -> (F, F, F, F, F, F, F) {
    let t1487 = t1483 * t1486;
    let t1491 = t328 * t407 * t209;
    let t1492 = t1491 * t430;
    let t1493 = t418 * t194;
    let t1495 = 1.0 / t196 / t1493;
    let t1496 = t414 * t1495;
    let t1497 = t423 * tau1;
    let t1498 = t1496 * t1497;
    let t1502 = 1.0 / t429 / t211;
    let t1503 = t428 * t1502;
    let t1514 = -0.125104062565404384e1 * t398 * t1297 * t401 + 0.58691349263882304531e0 * t1454 * t1304 * t1455 + 5.0 / 3.0 * t1459 * t1311 + 5.0 / 3.0 * t405 * t1315 + 10.0 / 3.0 * t1464 * t1315 + 10.0 / 3.0 * t408 * t1469 * t198 - 0.17058312527037532642e0 * t415 * t1475 + 0.80027407411602181738e-1 * t1480 * t1487 + 0.7107630219598971934e-1 * t1492 * t1498 + 0.7107630219598971934e-1 * t1503 * t1498 - 0.17058312527037532642e0 * t431 * t414 * t1474 * t423 + 0.80027407411602181738e-1 * t431 * t1479 * t1483 * t1486;
    (t1491, t1492, t1493, t1497, t1502, t1503, t1514)
}
