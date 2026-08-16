//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 826/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk826(t598: f64, t9577: f64, t604: f64, t6847: f64, t1181: f64, t2068: f64, t157: f64, t495: f64, t524: f64, t599: f64, t7337: f64, t6841: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9578 = t598 * t9577;
    let t9582 = t604 * t6847;
    let t9583 = t1181 * t9582;
    let t9584 = t2068 * t9583;
    let t9587 = t495 * t524 * t157;
    let t9588 = t599 * t9587;
    let t9589 = t1181 * t9588;
    let t9590 = t7337 * t9589;
    let t9592 = t604 * t6841;
    (t9578, t9582, t9583, t9584, t9587, t9588, t9589, t9590, t9592)
}
