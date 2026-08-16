//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1085/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1085(t4171: f64, t602: f64, t1466: f64, t2246: f64, t1497: f64, t2248: f64, t4241: f64, t644: f64, t2315: f64, t10355: f64, t1469: f64, t2251: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13269 = t4171 * t602;
    let t13272 = t1466 * t2246;
    let t13283 = t1497 * t2248;
    let t13286 = t4241 * t644;
    let t13289 = t1497 * t2315;
    let t13299 = t10355 * t1469 * t2251;
    (t13269, t13272, t13283, t13286, t13289, t13299)
}
