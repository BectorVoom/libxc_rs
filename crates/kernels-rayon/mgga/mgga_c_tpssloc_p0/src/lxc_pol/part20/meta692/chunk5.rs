//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2640/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2640(t113: f64, t12504: f64, t12507: f64, t12545: f64, t12557: f64, t1271: f64, t12841: f64, t16503: f64, t2314: f64, t2320: f64, t2363: f64, t4028: f64, t4034: f64, t4073: f64, t4077: f64, t45782: f64, t46118: f64, t50803: f64, t510: f64, t5107: f64, t53757: f64, t574: f64, t652: f64, t9348: f64) -> f64 {
    let t53774 = -2.0_f64 * t652 * t510 * t45782 - 6.0_f64 * t4034 * t12557 - 6.0_f64 * t652 * t5107 * t2363 - 6.0_f64 * t4028 * t12504 + t46118 * t574 - t113 * (t50803 + t53757) - 6.0_f64 * t4028 * t12507 - 6.0_f64 * t2314 * t12841 - 12.0_f64 * t2314 * t12545 - 6.0_f64 * t9348 * t4077 - 6.0_f64 * t9348 * t4073 - 6.0_f64 * t2320 * t5107 + 3.0_f64 * t1271 * t16503;
    t53774
}
