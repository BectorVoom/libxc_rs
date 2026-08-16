//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1090/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1090(t1462: f64, t7605: f64, t1998: f64, t4720: f64, t1298: f64, t7380: f64, t7381: f64, t1524: f64, t1983: f64, t2095: f64, t435: f64, t7815: f64) -> (f64, f64, f64, f64, f64) {
    let t35400 = t7605 * t1462;
    let t35403 = t1998 * t4720;
    let t35407 = t7380 * t7381 * t1298;
    let t35410 = t2095 * t1983 * t1524;
    let t35413 = t7815 * t435;
    (t35400, t35403, t35407, t35410, t35413)
}
