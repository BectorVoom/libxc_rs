//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1139/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1139(t1839: f64, t322: f64, t1181: f64, t599: f64, t7346: f64, t39499: f64, t301: f64, t7337: f64, t1859: f64, t372: f64, t7351: f64, t7575: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39743 = t1839 * t322;
    let t39746 = t7346 * t1181 * t599 * t39743;
    let t39750 = t7346 * t1181 * t599 * t39499;
    let t39753 = t1839 * t301;
    let t39756 = t7337 * t1181 * t599 * t39753;
    let t39763 = t7575 * t1181 * t7351 * t1859 * t372;
    (t39743, t39746, t39750, t39753, t39756, t39763)
}
