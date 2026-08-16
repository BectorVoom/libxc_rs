//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1887/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1887(t1466: f64, t2246: f64, t1497: f64, t2248: f64, t4241: f64, t644: f64, t2315: f64, t10355: f64, t1469: f64, t2251: f64, t2275: f64, t4186: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13272 = t1466 * t2246;
    let t13283 = t1497 * t2248;
    let t13286 = t4241 * t644;
    let t13289 = t1497 * t2315;
    let t13299 = t10355 * t1469 * t2251;
    let t13302 = t2275 * t4186;
    (t13272, t13283, t13286, t13289, t13299, t13302)
}
