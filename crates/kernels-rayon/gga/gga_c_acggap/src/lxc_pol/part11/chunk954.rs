//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 954/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk954(t31699: f64, t7588: f64, t2074: f64, t30797: f64, t7844: f64, t1111: f64, t1992: f64, t30147: f64, t7586: f64, t1165: f64, t30209: f64, t3044: f64, t604: f64) -> (f64, f64, f64, f64, f64) {
    let t31700 = t31699 * t7588;
    let t31702 = t30797 * t2074;
    let t31704 = t31699 * t7844;
    let t31708 = t30147 * t7586 * t1992 * t1111;
    let t31720 = t30209 * t1165 * t604 * t3044;
    (t31700, t31702, t31704, t31708, t31720)
}
