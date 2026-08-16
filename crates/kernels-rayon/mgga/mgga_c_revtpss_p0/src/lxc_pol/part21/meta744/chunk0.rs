//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2618/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2618(t13581: f64, t177: f64, t762: f64, t46971: f64, t1317: f64, t13632: f64, t3857: f64, t5569: f64, t512: f64, t749: f64, t46973: f64, t3863: f64, t5567: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48222 = t13581 * t177 * t762;
    let t48223 = 0.17544670867903938621e1_f64 * t48222;
    let t48224 = 480.0_f64 * t46971;
    let t48225 = t1317 * t13632;
    let t48226 = 12.0_f64 * t48225;
    let t48227 = t3857 * t5569;
    let t48228 = 60.0_f64 * t48227;
    let t48230 = t512 * t13581 * t749;
    let t48231 = 3.0_f64 * t48230;
    let t48232 = 36.0_f64 * t46973;
    let t48234 = 96.0_f64 * t3863 * t5567;
    (t48223, t48224, t48226, t48228, t48231, t48232, t48234)
}
