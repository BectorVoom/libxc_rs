//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 921/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk921(t1004: f64, t390: f64, t7613: f64, t1998: f64, t3786: f64, t151: f64, t37: f64, t56: f64, t593: f64, t7508: f64, t141: f64, t420: f64) -> (f64, f64, f64, f64) {
    let t31001 = t1004 * t7613 * t390;
    let t31003 = t1998 * t3786;
    let t31009 = t151 * t593 / t7508 / t37 * t56;
    let t31010 = t420 * t141;
    (t31001, t31003, t31009, t31010)
}
