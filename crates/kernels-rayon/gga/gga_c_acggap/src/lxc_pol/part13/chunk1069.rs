//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1069/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1069(t1165: f64, t30282: f64, t33911: f64, t604: f64, t1992: f64, t5616: f64, t7585: f64, t7586: f64, t1017: f64, t525: f64, t1181: f64, t2068: f64, t7351: f64) -> (f64, f64, f64, f64) {
    let t34671 = t30282 * t1165 * t604 * t33911;
    let t34675 = t7585 * t7586 * t1992 * t5616;
    let t34681 = t525 * t1017;
    let t34684 = t2068 * t1181 * t7351 * t34681;
    (t34671, t34675, t34681, t34684)
}
