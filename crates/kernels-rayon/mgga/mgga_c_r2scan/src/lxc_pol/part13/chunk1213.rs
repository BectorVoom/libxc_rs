//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1213/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1213(t32094: f64, t792: f64, t37327: f64, t4176: f64, t11502: f64, t37346: f64, t1561: f64, t3274: f64, t97: f64, t32212: f64, t14160: f64, t1234: f64, t2867: f64) -> (f64, f64, f64, f64) {
    let t40566 = t32094 * t792;
    let t40569 = 15.0_f64 / 8.0_f64 * t37327 * t4176 * t40566;
    let t40571 = 3.0_f64 / 4.0_f64 * t37346 * t11502;
    let t40574 = t97 * t3274 * t1561;
    let t40575 = t32212 * t792;
    let t40578 = 5.0_f64 / 4.0_f64 * t40574 * t14160 * t40575;
    let t40579 = t2867 * t1234;
    (t40569, t40571, t40578, t40579)
}
