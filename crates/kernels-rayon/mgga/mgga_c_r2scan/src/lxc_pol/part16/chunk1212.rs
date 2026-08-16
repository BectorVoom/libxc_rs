//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1212/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1212(t3602: f64, t39922: f64, t8081: f64, t37755: f64, t7619: f64, t40033: f64, t7624: f64, t3606: f64, t39935: f64, t1055: f64, t9085: f64, t30281: f64, t3332: f64, t7628: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43459 = t39922 * t3602 * t8081;
    let t43462 = t37755 * t3602 * t7619;
    let t43465 = t40033 * t3602 * t7624;
    let t43468 = t37755 * t3606 * t8081;
    let t43471 = t39935 * t3606 * t7619;
    let t43474 = t9085 * t1055;
    let t43477 = t7628 * t3332 * t30281;
    (t43459, t43462, t43465, t43468, t43471, t43474, t43477)
}
