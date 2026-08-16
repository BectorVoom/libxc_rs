//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2230/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2230(t13272: f64, t1470: f64, t28150: f64, t7715: f64, t29543: f64, t644: f64, t77: f64, t1497: f64, t7719: f64, t1926: f64, t29547: f64, t1927: f64, t5872: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t108966 = t13272 * t1470;
    let t108971 = t7715 * t28150;
    let t108975 = t77 * t29543 * t644;
    let t108978 = t7719 * t1497;
    let t108979 = t1926 * t108978;
    let t108983 = t77 * t29547 * t644;
    let t108986 = t1927 * t5872;
    (t108966, t108971, t108975, t108979, t108983, t108986)
}
