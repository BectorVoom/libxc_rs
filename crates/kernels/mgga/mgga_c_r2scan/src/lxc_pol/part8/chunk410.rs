//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 410/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk410<F: Float>(t1381: F, t1384: F, t1398: F, t1428: F, t1434: F, t1436: F, t1446: F, t1451: F, t1454: F, t1459: F, t1463: F, t1470: F, t1480: F, t1488: F, t1492: F, t1498: F, t1499: F, t1502: F, t1505: F, t432: F, t439: F, t447: F, t454: F, t5: F, t625: F, t72: F, t85: F) -> (F, F) {
    let t1506 = t1381 * t1384;
    let t1509 = -0.70983522622222222221e-3 * t5 * t1398 * t72 - 0.34246666666666666666e-1 * t625 * t1428 * t439 - 2.0 * t1434 * t1436 + 1.0 * t432 * t1446 + 0.32163958997385070134e2 * t1451 * t1454 + t1459 + t1463 + t1470 - t1480 - t1488 - 0.24415263074675393405e-3 * t5 * t1398 * t85 - 0.10843581300301739842e-1 * t625 * t1492 * t454 - 0.11696447245269292414e1 * t1498 * t1499 + 0.5848223622634646207e0 * t447 * t1502 + 0.17315859105681463759e2 * t1505 * t1506;
    (t1506, t1509)
}
