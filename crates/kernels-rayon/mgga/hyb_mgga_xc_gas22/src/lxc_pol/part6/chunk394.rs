//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 394/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk394(t1502: f64, t428: f64, t1297: f64, t1304: f64, t1311: f64, t1315: f64, t1454: f64, t1455: f64, t1459: f64, t1464: f64, t1469: f64, t1474: f64, t1475: f64, t1479: f64, t1480: f64, t1483: f64, t1486: f64, t1487: f64, t1492: f64, t1498: f64, t198: f64, t398: f64, t401: f64, t405: f64, t408: f64, t414: f64, t415: f64, t423: f64, t431: f64) -> (f64, f64) {
    let t1503 = t428 * t1502;
    let t1514 = -0.125104062565404384e1_f64 * t398 * t1297 * t401 + 0.58691349263882304531e0_f64 * t1454 * t1304 * t1455 + 5.0_f64 / 3.0_f64 * t1459 * t1311 + 5.0_f64 / 3.0_f64 * t405 * t1315 + 10.0_f64 / 3.0_f64 * t1464 * t1315 + 10.0_f64 / 3.0_f64 * t408 * t1469 * t198 - 0.17058312527037532642e0_f64 * t415 * t1475 + 0.80027407411602181738e-1_f64 * t1480 * t1487 + 0.7107630219598971934e-1_f64 * t1492 * t1498 + 0.7107630219598971934e-1_f64 * t1503 * t1498 - 0.17058312527037532642e0_f64 * t431 * t414 * t1474 * t423 + 0.80027407411602181738e-1_f64 * t431 * t1479 * t1483 * t1486;
    (t1503, t1514)
}
