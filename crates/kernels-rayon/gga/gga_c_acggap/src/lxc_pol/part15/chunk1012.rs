//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1012/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1012(t30374: f64, t8606: f64, t7426: f64, t7569: f64, t8480: f64, t7433: f64, t8481: f64, t34161: f64, t8465: f64, t31421: f64, t1992: f64, t7585: f64, t7842: f64, t8402: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35587 = t30374 * t8606;
    let t35594 = t7426 * t8480 * t7569;
    let t35596 = t7433 * t8481;
    let t35601 = t34161 * t8465;
    let t35603 = 0.22921875e-1_f64 * t31421;
    let t35608 = t7585 * t7842 * t1992 * t8402;
    (t35587, t35594, t35596, t35601, t35603, t35608)
}
