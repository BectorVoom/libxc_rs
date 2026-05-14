//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 393/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk393<F: Float>(t211: F, t416: F, t209: F, t322: F, t414: F, t422: F, t403: F, t424: F, t428: F, t421: F, t420: F, t400: F, t1305: F, t1309: F, t1447: F, t1448: F, t1452: F, t1458: F, t1461: F, t1466: F, t198: F, t397: F, t398: F, t408: F, t412: F, t415: F, t423: F, rho1: F, tau1: F) -> (F, F, F, F, F, F, F, F) {
    let t1469 = t416 * t211;
    let t1470 = 1.0 / t1469;
    let t1471 = t1470 * tau1;
    let t1476 = t322 * t414 * t209;
    let t1477 = t1476 * t422;
    let t1478 = t424 * t403;
    let t1479 = t428 * tau1;
    let t1480 = t1478 * t1479;
    let t1484 = 1.0 / t421 / t211;
    let t1485 = t420 * t1484;
    let t1488 = t400 * rho1;
    let t1489 = 1.0 / t1488;
    let t1498 = -0.17066666666666666667e-1 * t398 * t1448 + 0.34133333333333333333e-2 * t1452 * t1458 + 5.0 / 3.0 * t1461 * t1305 + 5.0 / 3.0 * t412 * t1309 + 10.0 / 3.0 * t1466 * t1309 + 10.0 / 3.0 * t415 * t1471 * t198 + 0.53333333333333333333e-1 * t1477 * t1480 + 0.53333333333333333333e-1 * t1485 * t1480 - 0.64e-1 * t423 * t424 * t1489 * t428 + 0.128e-1 * t423 * t397 * t1447 * t408;
    (t1470, t1471, t1476, t1477, t1479, t1484, t1485, t1498)
}
