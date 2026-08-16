//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2697/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2697(t40041: f64, t544: f64, t68: f64, t1332: f64, t16046: f64, t1352: f64, t3850: f64, t12169: f64, t12178: f64, t12259: f64, t12273: f64, t12435: f64, t1336: f64, t16033: f64, t16068: f64, t16132: f64, t16433: f64, t1814: f64, t1838: f64, t19810: f64, t3777: f64, t3851: f64, t3856: f64, t40118: f64, t5234: f64, t5287: f64, t5335: f64, t5344: f64, t5348: f64) -> (f64, f64, f64) {
    let t54963 = t544 * t68 * t40041;
    let t54976 = t1332 * t16046;
    let t55003 = t1352 * t3850;
    let t55012 = -t12178 * t1336 * t5348 - 3.0_f64 * t12259 * t1336 * t5287 - 3.0_f64 * t1336 * t16132 * t3851 - 3.0_f64 * t1336 * t16132 * t3856 - 3.0_f64 * t5335 * t5344 * t55003 - t12169 * t5234 - 3.0_f64 * t12273 * t19810 + t12435 * t1814 - 6.0_f64 * t16033 * t16068 - 3.0_f64 * t16433 * t3777 - t1838 * t40118;
    (t54963, t54976, t55012)
}
