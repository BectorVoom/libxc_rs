//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 516/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk516(t1216: f64, t2372: f64, t1268: f64, t2359: f64, t2363: f64, t2369: f64, t295: f64, t305: f64, t803: f64, t811: f64, t991: f64, t997: f64) -> (f64, f64) {
    let t2373 = t2372 * t1216;
    let t2376 = -25.0_f64 / 9.0_f64 * t803 * t991 + 10.0_f64 / 9.0_f64 * t295 * t2359 + 5.0_f64 / 3.0_f64 * t295 * t2363 - 25.0_f64 / 9.0_f64 * t997 * t811 + 10.0_f64 / 9.0_f64 * t305 * t2369 - 5.0_f64 / 3.0_f64 * t305 * t2373 - t1268;
    (t2373, t2376)
}
