//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1006/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1006(t3362: f64, t3698: f64, t2251: f64, t1012: f64, t1251: f64, t3172: f64) -> (f64, f64, f64) {
    let t3699 = t3698 * t3362;
    let t3700 = t3699 * t2251;
    let t3701 = t1012 * t3700;
    let t3704 = t3172 * t1251;
    (t3700, t3701, t3704)
}
