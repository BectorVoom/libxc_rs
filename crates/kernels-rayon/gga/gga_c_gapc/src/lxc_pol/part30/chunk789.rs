//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 789/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk789(t916: f64, t9386: f64, t128: f64, t6: f64, t442: f64, t919: f64, t1081: f64, t2645: f64, t7451: f64, t8673: f64, t6182: f64, t8676: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9387 = t916 * t9386;
    let t9388 = t6 * t128;
    let t9389 = t9388 * t442;
    let t9390 = t919 * t9389;
    let t9391 = t9387 * t9390;
    let t9393 = t1081 * t2645;
    let t9395 = t7451 * t8673;
    let t9396 = t8676 * t6182;
    (t9387, t9388, t9391, t9393, t9395, t9396)
}
