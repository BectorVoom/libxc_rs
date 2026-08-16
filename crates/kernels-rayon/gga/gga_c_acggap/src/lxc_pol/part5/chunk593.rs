//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 593/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk593(t164: f64, t177: f64, t3646: f64, t360: f64, t864: f64, t368: f64, t398: f64, t1036: f64, t372: f64, t1095: f64, t1165: f64, t388: f64, t955: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3649 = 0.21437009059034868486e-3_f64 * t3646 * t164 * t177;
    let t3650 = t864 * t360;
    let t3652 = t398 * t368 * t3650;
    let t3653 = t1036 * t3652;
    let t3655 = t864 * t372;
    let t3657 = t398 * t1095 * t3655;
    let t3658 = t1036 * t3657;
    let t3665 = t1165 * t388 * t955;
    (t3649, t3650, t3652, t3653, t3655, t3657, t3658, t3665)
}
