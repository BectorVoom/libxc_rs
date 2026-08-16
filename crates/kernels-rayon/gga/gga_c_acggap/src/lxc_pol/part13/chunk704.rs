//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 704/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk704(t598: f64, t7523: f64, t1083: f64, t355: f64, t360: f64, t7458: f64, t1980: f64, t1988: f64, t2113: f64, t1131: f64, t137: f64, t1089: f64, t1459: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7524 = t598 * t7523;
    let t7528 = t7458 * t1083 * t355 * t360;
    let t7529 = t1980 * t7528;
    let t7531 = t1988 * t2113;
    let t7533 = t137 * t1131;
    let t7535 = t1089 * t1459 * t7533;
    (t7524, t7528, t7529, t7531, t7533, t7535)
}
