//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 364/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk364(t377: f64, t446: f64, t445: f64, t81: f64, t76: f64, t1381: f64, t453: f64, t1409: f64, t1380: f64, t1384: f64, t1398: f64, t1428: f64, t1434: f64, t1436: f64, t1446: f64, t1451: f64, t1454: f64, t1459: f64, t1463: f64, t1470: f64, t1480: f64, t1488: f64, t432: f64, t439: f64, t447: f64, t454: f64, t5: f64, t625: f64, t72: f64, t85: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1492 = t377 * t446;
    let t1496 = t445 * t81;
    let t1497 = 1.0_f64 / t1496;
    let t1498 = t76 * t1497;
    let t1499 = t1381 * t453;
    let t1502 = t1409 * t453;
    let t1505 = t76 * t1380;
    let t1506 = t1381 * t1384;
    let t1509 = -0.70983522622222222221e-3_f64 * t5 * t1398 * t72 - 0.34246666666666666666e-1_f64 * t625 * t1428 * t439 - 2.0_f64 * t1434 * t1436 + 1.0_f64 * t432 * t1446 + 0.32163958997385070134e2_f64 * t1451 * t1454 + t1459 + t1463 + t1470 - t1480 - t1488 - 0.24415263074675393405e-3_f64 * t5 * t1398 * t85 - 0.10843581300301739842e-1_f64 * t625 * t1492 * t454 - 0.11696447245269292414e1_f64 * t1498 * t1499 + 0.5848223622634646207e0_f64 * t447 * t1502 + 0.17315859105681463759e2_f64 * t1505 * t1506;
    (t1492, t1497, t1498, t1499, t1502, t1505, t1506, t1509)
}
