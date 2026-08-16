//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 828/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk828(t2068: f64, t9633: f64, t1839: f64, t599: f64, t1181: f64, t1165: f64, t604: f64, t1815: f64, t7413: f64, t1849: f64, t7351: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9634 = t2068 * t9633;
    let t9636 = t599 * t1839;
    let t9637 = t1181 * t9636;
    let t9638 = t2068 * t9637;
    let t9641 = t1165 * t604 * t1839;
    let t9642 = t2068 * t9641;
    let t9645 = t1165 * t604 * t1815;
    let t9646 = t7413 * t9645;
    let t9648 = t599 * t1815;
    let t9649 = t1181 * t9648;
    let t9650 = t7413 * t9649;
    let t9653 = t1165 * t7351 * t1849;
    (t9634, t9636, t9637, t9638, t9641, t9642, t9645, t9646, t9648, t9649, t9650, t9653)
}
