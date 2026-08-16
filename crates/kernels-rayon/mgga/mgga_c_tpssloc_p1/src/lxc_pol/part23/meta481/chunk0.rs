//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1439/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1439(t15376: f64, t22069: f64, t3447: f64, t4908: f64, t6123: f64, t64811: f64, t73274: f64, t73276: f64, t73279: f64, t73287: f64, t73290: f64, t73307: f64, t73314: f64, t78043: f64, t78047: f64) -> f64 {
    let t78441 = -0.1086419753086419753e-1_f64 * t73274 + 0.59259259259259259256e-2_f64 * t73276 - 0.11522633744855967078e-2_f64 * t73279 - 0.37037037037037037036e-3_f64 * t73287 - 0.33333333333333333332e-2_f64 * t73290 + 0.29629629629629629628e-2_f64 * t73307 + 0.29629629629629629628e-2_f64 * t73314 - 0.22222222222222222221e-2_f64 * t3447 * t4908 * t78047 - 0.99999999999999999996e-2_f64 * t3447 * t4908 * t78043 + 0.32592592592592592592e-1_f64 * t64811 * t6123 - 0.88888888888888888887e-2_f64 * t15376 * t22069;
    t78441
}
