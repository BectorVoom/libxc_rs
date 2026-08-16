//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1120/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1120(t1181: f64, t26757: f64, t599: f64, t7413: f64, t6237: f64, t7561: f64, t7433: f64, t9633: f64, t30371: f64, t5940: f64, t7575: f64, t8480: f64, t8609: f64) -> (f64, f64, f64, f64, f64) {
    let t39362 = t7413 * t1181 * t599 * t26757;
    let t39364 = t7561 * t6237;
    let t39366 = t7433 * t9633;
    let t39368 = t30371 * t5940;
    let t39373 = t7575 * t8480 * t8609;
    (t39362, t39364, t39366, t39368, t39373)
}
