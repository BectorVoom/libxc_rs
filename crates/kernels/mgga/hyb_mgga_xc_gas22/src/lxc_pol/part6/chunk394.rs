//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 394/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk394<F: Float>(t1502: F, t428: F, t1297: F, t1304: F, t1311: F, t1315: F, t1454: F, t1455: F, t1459: F, t1464: F, t1469: F, t1474: F, t1475: F, t1479: F, t1480: F, t1483: F, t1486: F, t1487: F, t1492: F, t1498: F, t198: F, t398: F, t401: F, t405: F, t408: F, t414: F, t415: F, t423: F, t431: F) -> (F, F) {
    let t1503 = t428 * t1502;
    let t1514 = -F::cast_from(0.125104062565404384e1_f64) * t398 * t1297 * t401 + F::cast_from(0.58691349263882304531e0_f64) * t1454 * t1304 * t1455 + F::new(5.0) / F::new(3.0) * t1459 * t1311 + F::new(5.0) / F::new(3.0) * t405 * t1315 + F::new(10.0) / F::new(3.0) * t1464 * t1315 + F::new(10.0) / F::new(3.0) * t408 * t1469 * t198 - F::cast_from(0.17058312527037532642e0_f64) * t415 * t1475 + F::cast_from(0.80027407411602181738e-1_f64) * t1480 * t1487 + F::cast_from(0.7107630219598971934e-1_f64) * t1492 * t1498 + F::cast_from(0.7107630219598971934e-1_f64) * t1503 * t1498 - F::cast_from(0.17058312527037532642e0_f64) * t431 * t414 * t1474 * t423 + F::cast_from(0.80027407411602181738e-1_f64) * t431 * t1479 * t1483 * t1486;
    (t1503, t1514)
}
