//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 890/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk890(t30546: f64, t7570: f64, t1106: f64, t1992: f64, t30147: f64, t7586: f64, t7478: f64, t7799: f64, t3176: f64, t7585: f64, t1004: f64, t1966: f64) -> (f64, f64, f64, f64, f64) {
    let t30547 = t30546 * t7570;
    let t30559 = t30147 * t7586 * t1992 * t1106;
    let t30561 = t7799 * t7478;
    let t30565 = t7585 * t7586 * t1992 * t3176;
    let t30567 = t1004 * t1966;
    (t30547, t30559, t30561, t30565, t30567)
}
