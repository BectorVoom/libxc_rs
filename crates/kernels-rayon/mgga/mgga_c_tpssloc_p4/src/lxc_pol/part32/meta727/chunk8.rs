//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2361/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2361(t20100: f64, t20136: f64, t20143: f64, t2314: f64, t24932: f64, t27888: f64, t29855: f64, t4034: f64, t5450: f64, t5494: f64, t6287: f64, t6468: f64, t7264: f64, t7266: f64, t7408: f64, t7412: f64, t97899: f64, t97905: f64, t97910: f64, t97914: f64, t97916: f64, t97919: f64, t97923: f64, t97925: f64, t97928: f64) -> f64 {
    let t105092 = -2.0_f64 * t20100 * t7266 - 4.0_f64 * t20136 * t7266 - 2.0_f64 * t20143 * t7266 - 2.0_f64 * t2314 * t29855 - 2.0_f64 * t24932 * t5494 - 2.0_f64 * t27888 * t5494 - 2.0_f64 * t29855 * t4034 - t5450 * t7408 - t6287 * t7264 + t6468 * t7412 + t97899 - t97905 - t97910 + t97914 - t97916 - t97919 + t97923 + t97925 - t97928;
    t105092
}
