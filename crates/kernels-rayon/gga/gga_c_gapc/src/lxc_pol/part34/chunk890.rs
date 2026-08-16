//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 890/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk890(t10133: f64, t3192: f64, t10103: f64, t10106: f64, t10108: f64, t10111: f64, t10115: f64, t10118: f64, t10120: f64, t10126: f64, t10128: f64, t10131: f64) -> (f64, f64) {
    let t10134 = t3192 * t10133;
    let t10136 = -0.27357942622625364862e-5_f64 * t10103 + 0.13223005600935593017e-4_f64 * t10106 + 0.1252584660908875509e-2_f64 * t10108 + 0.11742981196020707897e-5_f64 * t10111 + 0.18788769913633132635e-4_f64 * t10115 + 0.27357942622625364862e-5_f64 * t10118 + 0.94840867758434598187e-4_f64 * t10120 + 0.19798879235883268025e-5_f64 * t10126 + 0.14615314396567373048e-4_f64 * t10128 - 0.28183154870449698953e-3_f64 * t10131 - 0.18788769913633132635e-4_f64 * t10134;
    (t10134, t10136)
}
