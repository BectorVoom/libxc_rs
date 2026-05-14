//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 352/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk352<F: Float>(t1422: F, t89: F, t377: F, t431: F, t430: F, t68: F, t63: F, t437: F, t438: F, t1399: F, t1402: F, t1404: F, t1407: F, t1393: F, t1396: F, t71: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t1423 = t1422 * t89;
    let t1424 = 32.0 * t1423;
    let t1428 = t377 * t431;
    let t1432 = t430 * t68;
    let t1433 = 1.0 / t1432;
    let t1434 = t63 * t1433;
    let t1435 = t437 * t437;
    let t1436 = t1435 * t438;
    let t1441 = 0.68863333333333333333e0 * t1399;
    let t1442 = 0.14025833333333333333e0 * t1402;
    let t1443 = 0.28051666666666666667e0 * t1404;
    let t1444 = 0.17365833333333333333e0 * t1407;
    let t1445 = -0.78438333333333333333e0 * t1393 + 0.15687666666666666667e1 * t1396 + t1441 + t1442 + t1443 + t1444;
    let t1446 = t1445 * t438;
    let t1449 = t430 * t430;
    let t1450 = 1.0 / t1449;
    let t1451 = t63 * t1450;
    let t1452 = t71 * t71;
    (t1424, t1428, t1433, t1434, t1435, t1436, t1441, t1442, t1443, t1444, t1445, t1446, t1449, t1450, t1451, t1452)
}
