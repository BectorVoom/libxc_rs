//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1133/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1133(t31382: f64, t31386: f64, t31346: f64, t4912: f64, t7413: f64, t7835: f64, t8480: f64, t30219: f64, t8446: f64, t1439: f64, t30148: f64, t30154: f64, t7842: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35562 = 13.0_f64 / 48.0_f64 * t31382;
    let t35563 = 0.85748036236139473944e-3_f64 * t31386;
    let t35564 = t31346 * t4912;
    let t35567 = t7413 * t8480 * t7835;
    let t35569 = t30219 * t8446;
    let t35570 = 0.31448092289604152068e-2_f64 * t35569;
    let t35573 = t30154 * t7842 * t30148 * t1439;
    (t35562, t35563, t35564, t35567, t35570, t35573)
}
