//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1520/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1520<F: Float>(t80265: F, t80303: F, t80330: F, t80352: F, t80375: F, t80399: F, t80442: F, t80474: F, t1336: F, t1825: F, t1838: F, t19657: F, t19815: F, t20490: F, t20553: F, t20622: F, t20630: F, t3792: F, t5234: F, t5334: F, t5335: F, t5344: F, t544: F, t54930: F, t553: F, t6420: F, t6451: F, t6456: F, t74289: F, t74937: F, t74949: F) -> (F, F) {
    let t80477 = t80265 + t80303 + t80330 + t80352 + t80375 + t80399 + t80442 + t80474;
    let t80482 = F::cast_from(8.0_f64) * t20553 * t3792 * t5334 * t5335 - F::cast_from(6.0_f64) * t1336 * t19657 * t6420 - F::cast_from(24.0_f64) * t1336 * t20490 * t54930 - F::cast_from(12.0_f64) * t1825 * t5344 * t74937 - F::cast_from(4.0_f64) * t1825 * t5344 * t74949 + t544 * t553 * t80477 - F::cast_from(4.0_f64) * t1838 * t74289 - F::cast_from(12.0_f64) * t19815 * t6451 - F::cast_from(6.0_f64) * t19815 * t6456 - F::cast_from(24.0_f64) * t20622 * t5234 - F::cast_from(4.0_f64) * t20630 * t5234;
    (t80477, t80482)
}
