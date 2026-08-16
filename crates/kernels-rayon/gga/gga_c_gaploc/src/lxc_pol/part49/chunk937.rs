//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 937/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk937(t12782: f64, t64: f64, t10205: f64, t871: f64, t39624: f64, t39626: f64, t39632: f64, t39646: f64, t39648: f64, t39650: f64, t1: f64, t1415: f64, t2413: f64, t31730: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42113 = 4.0_f64 / 3.0_f64 * t12782 * t64;
    let t42114 = t10205 * t871;
    let t42117 = 7.0_f64 / 512.0_f64 * t39624;
    let t42118 = 63.0_f64 / 16384.0_f64 * t39626;
    let t42119 = 63.0_f64 / 1048576.0_f64 * t39632;
    let t42120 = 21.0_f64 / 1048576.0_f64 * t39646;
    let t42121 = 21.0_f64 / 16384.0_f64 * t39648;
    let t42122 = 7.0_f64 / 1536.0_f64 * t39650;
    let t42138 = t1415 * t31730 * t1 * t2413;
    (t42113, t42114, t42117, t42118, t42119, t42120, t42121, t42122, t42138)
}
