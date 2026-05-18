//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 370/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk370<F: Float>(t1407: F, t1393: F, t1396: F, t1441: F, t1442: F, t1443: F, t438: F, t430: F, t63: F, t71: F, t1435: F, t1398: F, t32: F, t5: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1444 = F::new(0.17365833333333333333e0) * t1407;
    let t1445 = -F::new(0.78438333333333333333e0) * t1393 + F::new(0.15687666666666666667e1) * t1396 + t1441 + t1442 + t1443 + t1444;
    let t1446 = t1445 * t438;
    let t1449 = t430 * t430;
    let t1450 = F::new(1.0) / t1449;
    let t1451 = t63 * t1450;
    let t1452 = t71 * t71;
    let t1453 = F::new(1.0) / t1452;
    let t1454 = t1435 * t1453;
    let t1458 = t5 * t1398 * t32;
    (t1444, t1445, t1446, t1449, t1450, t1451, t1452, t1453, t1454, t1458)
}
