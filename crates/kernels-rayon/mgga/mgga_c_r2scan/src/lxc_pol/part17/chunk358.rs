//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 358/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk358(t1407: f64, t1393: f64, t1396: f64, t1441: f64, t1442: f64, t1443: f64, t438: f64, t430: f64, t63: f64, t71: f64, t1435: f64, t1398: f64, t32: f64, t5: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1444 = 0.17365833333333333333e0_f64 * t1407;
    let t1445 = -0.78438333333333333333e0_f64 * t1393 + 0.15687666666666666667e1_f64 * t1396 + t1441 + t1442 + t1443 + t1444;
    let t1446 = t1445 * t438;
    let t1449 = t430 * t430;
    let t1450 = 1.0_f64 / t1449;
    let t1451 = t63 * t1450;
    let t1452 = t71 * t71;
    let t1453 = 1.0_f64 / t1452;
    let t1454 = t1435 * t1453;
    let t1458 = t5 * t1398 * t32;
    (t1444, t1445, t1446, t1449, t1450, t1451, t1452, t1453, t1454, t1458)
}
