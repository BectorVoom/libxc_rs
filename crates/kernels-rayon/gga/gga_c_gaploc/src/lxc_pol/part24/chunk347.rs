//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 347/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk347(t110: f64, t386: f64, t1094: f64, t1121: f64, t513: f64, t10: f64, t101: f64, t107: f64, t1208: f64, t1465: f64, t1469: f64, t1475: f64, t1478: f64, t1482: f64, t1484: f64, t1488: f64, t179: f64, t180: f64, t183: f64, t415: f64, t503: f64, t507: f64, t510: f64, t514: f64, t79: f64) -> f64 {
    let t1491 = t110 * t386;
    let t1492 = t1491 * t1094;
    let t1495 = t513 * t1121;
    let t1507 = 0.619125e-2_f64 * t1465 * t180 - 0.24765e-1_f64 * t1469 * t510 - 0.123825e-1_f64 * t503 * t514 + 0.206375e-2_f64 * t1475 * t1478 + 0.24765e-1_f64 * t1482 * t1484 + 0.1651e-1_f64 * t507 * t1488 + 0.123825e-1_f64 * t179 * t1492 - 0.619125e-2_f64 * t179 * t1495 + 0.17687407407407407407e-1_f64 * t107 * t79 * t101 - 0.10612444444444444444e0_f64 * t107 * t10 * t415 - 0.79593333333333333331e-1_f64 * t107 * t183 * t1208;
    t1507
}
