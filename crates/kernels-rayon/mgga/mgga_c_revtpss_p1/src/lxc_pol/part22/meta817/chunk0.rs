//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2927/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2927(t1904: f64, t2439: f64, t9640: f64, t5718: f64, t9292: f64, t14274: f64, t2435: f64, t4078: f64, t5599: f64, t689: f64, t13734: f64, t1445: f64) -> (f64, f64, f64, f64, f64) {
    let t47800 = t2439 * t9640 * t1904;
    let t47802 = t9292 * t5718;
    let t47805 = t2435 * t14274;
    let t47808 = t689 * t5599 * t4078;
    let t47811 = t689 * t13734 * t1445;
    (t47800, t47802, t47805, t47808, t47811)
}
