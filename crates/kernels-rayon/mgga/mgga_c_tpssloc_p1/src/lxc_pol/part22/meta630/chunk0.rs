//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2165/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2165(t54428: f64, t39571: f64, t39581: f64, t2225: f64, t5168: f64, t5154: f64, t9892: f64, t39601: f64, t39605: f64, t39607: f64, t39609: f64, t39634: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t54429 = 36.0_f64 * t54428;
    let t54430 = 144.0_f64 * t39571;
    let t54431 = 8.0_f64 * t39581;
    let t54432 = t2225 * t5168;
    let t54434 = t5154 * t9892;
    let t54436 = 12.0_f64 * t39601;
    let t54437 = 960.0_f64 * t39605;
    let t54438 = 192.0_f64 * t39607;
    let t54439 = 240.0_f64 * t39609;
    let t54447 = 48.0_f64 * t39634;
    (t54429, t54430, t54431, t54432, t54434, t54436, t54437, t54438, t54439, t54447)
}
