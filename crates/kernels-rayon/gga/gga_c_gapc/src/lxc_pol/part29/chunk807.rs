//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 807/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk807(t442: f64, t9388: f64, t919: f64, t9387: f64, t1081: f64, t2645: f64, t7451: f64, t8673: f64, t6182: f64, t8676: f64, t1084: f64, t8906: f64) -> (f64, f64, f64, f64, f64) {
    let t9389 = t9388 * t442;
    let t9390 = t919 * t9389;
    let t9391 = t9387 * t9390;
    let t9393 = t1081 * t2645;
    let t9395 = t7451 * t8673;
    let t9396 = t8676 * t6182;
    let t9397 = t9395 * t9396;
    let t9399 = t1084 * t8906;
    (t9391, t9393, t9396, t9397, t9399)
}
