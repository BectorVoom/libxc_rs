//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1037/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1037(t2363: f64, t89: f64, t12545: f64, t12550: f64, t12557: f64, t12725: f64, t12734: f64, t12816: f64, t1442: f64, t1459: f64, t1849: f64, t2314: f64, t2323: f64, t2364: f64, t3652: f64, t3660: f64, t4028: f64, t4034: f64, t4037: f64, t4073: f64, t574: f64, t652: f64, t672: f64, t9348: f64) -> (f64, f64) {
    let t12823 = t89 * t2363;
    let t12832 = -4.0_f64 * t12545 * t652 - 4.0_f64 * t12550 * t652 - 2.0_f64 * t12557 * t652 - 4.0_f64 * t12725 * t672 - 4.0_f64 * t12734 * t1459 + t12816 * t574 - 2.0_f64 * t12823 * t1459 - t1442 * t3652 - 2.0_f64 * t1459 * t9348 + t1849 * t3660 - 4.0_f64 * t2314 * t4073 - 4.0_f64 * t2323 * t4028 - 2.0_f64 * t2364 * t4028 - 4.0_f64 * t4034 * t4037 - 4.0_f64 * t4034 * t4073;
    (t12823, t12832)
}
