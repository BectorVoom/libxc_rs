//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1145/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1145(t12220: f64, t12223: f64, t31510: f64, t795: f64, t105: f64, t3052: f64, t97: f64, t2526: f64, t3574: f64, t2850: f64, t6967: f64, t106: f64, t8691: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42372 = 15.0_f64 / 8.0_f64 * t12220;
    let t42373 = t12223 / 2.0_f64;
    let t42384 = t31510 * t795;
    let t42389 = t97 * t105 * t3052;
    let t42392 = t3574 * t2526;
    let t42403 = t6967 * t2850;
    let t42413 = t97 * t106 * t8691;
    (t42372, t42373, t42384, t42389, t42392, t42403, t42413)
}
