//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1183/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1183(t1817: f64, t31863: f64, t1896: f64, t7614: f64, t1866: f64, t361: f64, t7436: f64, t142: f64, t6304: f64, t1998: f64, t5971: f64, t1426: f64, t1894: f64, t2085: f64, t598: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40308 = t31863 * t1817;
    let t40310 = t7614 * t1896;
    let t40313 = t7436 * t361 * t1866;
    let t40316 = t7436 * t142 * t6304;
    let t40318 = t1998 * t5971;
    let t40322 = t598 * t1426 * t1894 * t2085;
    (t40308, t40310, t40313, t40316, t40318, t40322)
}
