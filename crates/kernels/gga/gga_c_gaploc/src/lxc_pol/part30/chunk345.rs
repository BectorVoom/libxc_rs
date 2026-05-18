//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 345/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk345<F: Float>(t110: F, t386: F, t1094: F, t1121: F, t513: F, t10: F, t101: F, t107: F, t1208: F, t1465: F, t1469: F, t1475: F, t1478: F, t1482: F, t1484: F, t1488: F, t179: F, t180: F, t183: F, t415: F, t503: F, t507: F, t510: F, t514: F, t79: F) -> F {
    let t1491 = t110 * t386;
    let t1492 = t1491 * t1094;
    let t1495 = t513 * t1121;
    let t1507 = F::new(0.619125e-2) * t1465 * t180 - F::new(0.24765e-1) * t1469 * t510 - F::new(0.123825e-1) * t503 * t514 + F::new(0.206375e-2) * t1475 * t1478 + F::new(0.24765e-1) * t1482 * t1484 + F::new(0.1651e-1) * t507 * t1488 + F::new(0.123825e-1) * t179 * t1492 - F::new(0.619125e-2) * t179 * t1495 + F::new(0.17687407407407407407e-1) * t107 * t79 * t101 - F::new(0.10612444444444444444e0) * t107 * t10 * t415 - F::new(0.79593333333333333331e-1) * t107 * t183 * t1208;
    t1507
}
