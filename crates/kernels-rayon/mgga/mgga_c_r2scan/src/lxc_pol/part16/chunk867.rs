//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 867/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk867(t551: f64, t552: f64, t9124: f64, t9129: f64, t8692: f64, t2719: f64, t910: f64, t2526: f64, t938: f64, t1632: f64, t3056: f64, t574: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9190 = t551 * t552 * t9124;
    let t9194 = t551 * t552 * t9129;
    let t9202 = t551 * t552 * t8692;
    let t9207 = t2719 * t910;
    let t9209 = t551 * t552 * t9207;
    let t9212 = t938 * t2526;
    let t9214 = t551 * t552 * t9212;
    let t9218 = t551 * t1632 * t3056;
    let t9219 = t574 * t9218;
    (t9190, t9194, t9202, t9209, t9214, t9219)
}
