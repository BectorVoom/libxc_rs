//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1520/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1520(t80265: f64, t80303: f64, t80330: f64, t80352: f64, t80375: f64, t80399: f64, t80442: f64, t80474: f64, t1336: f64, t1825: f64, t1838: f64, t19657: f64, t19815: f64, t20490: f64, t20553: f64, t20622: f64, t20630: f64, t3792: f64, t5234: f64, t5334: f64, t5335: f64, t5344: f64, t544: f64, t54930: f64, t553: f64, t6420: f64, t6451: f64, t6456: f64, t74289: f64, t74937: f64, t74949: f64) -> (f64, f64) {
    let t80477 = t80265 + t80303 + t80330 + t80352 + t80375 + t80399 + t80442 + t80474;
    let t80482 = 8.0_f64 * t20553 * t3792 * t5334 * t5335 - 6.0_f64 * t1336 * t19657 * t6420 - 24.0_f64 * t1336 * t20490 * t54930 - 12.0_f64 * t1825 * t5344 * t74937 - 4.0_f64 * t1825 * t5344 * t74949 + t544 * t553 * t80477 - 4.0_f64 * t1838 * t74289 - 12.0_f64 * t19815 * t6451 - 6.0_f64 * t19815 * t6456 - 24.0_f64 * t20622 * t5234 - 4.0_f64 * t20630 * t5234;
    (t80477, t80482)
}
