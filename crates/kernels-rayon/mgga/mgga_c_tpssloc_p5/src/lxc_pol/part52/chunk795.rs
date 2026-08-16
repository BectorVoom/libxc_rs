//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 795/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk795(t1198: f64, t1218: f64, t1232: f64, t2134: f64, t2136: f64, t488: f64, t7309: f64, t7310: f64, t7315: f64, t7316: f64, t7321: f64, t7326: f64, t7331: f64, t7334: f64, t7339: f64, t7343: f64, t7345: f64) -> f64 {
    let t7348 = t7309 - t7310 * t1198 / 288.0_f64 + t7315 - 0.10093189023535097714e-3_f64 * t7316 * t2136 - 0.10093189023535097714e-3_f64 * t2134 * t7321 + 0.10093189023535097714e-3_f64 * t7326 * t7331 + t7334 * t488 / 1536.0_f64 + t7339 * t1218 / 1536.0_f64 + t7343 - t7345 * t1232 / 2304.0_f64;
    t7348
}
