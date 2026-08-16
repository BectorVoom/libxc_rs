//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1033/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1033(t140: f64, t3698: f64, t1012: f64, t13026: f64, t1234: f64, t5390: f64, t1802: f64, t3147: f64, t3597: f64, t3594: f64, t1244: f64, t12268: f64, t3617: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17471 = t140 * t3698;
    let t17475 = t1012 * t13026;
    let t17505 = t1234 * t5390;
    let t17523 = t1802 * t3147;
    let t17524 = t3597 * t17523;
    let t17525 = t3594 * t17524;
    let t17528 = t1244 * t17523;
    let t17529 = t3594 * t17528;
    let t17550 = t3617 * t12268;
    (t17471, t17475, t17505, t17524, t17525, t17528, t17529, t17550)
}
