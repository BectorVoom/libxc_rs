//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1785/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1785(t123: f64, t125: f64, t1358: f64, t555: f64, t8779: f64, t9645: f64, t1445: f64, t689: f64, t9634: f64, t2435: f64, t9667: f64, t268: f64, t39644: f64, t556: f64, t561: f64) -> (f64, f64, f64, f64) {
    let t47591 = 0.65457331274007190912e-5_f64 * t123 * t125 * t8779 * t9645 * t555 * t1358;
    let t47593 = t689 * t9634 * t1445;
    let t47595 = t2435 * t9667;
    let t47601 = 0.11638313500518478545e-4_f64 * t39644 * t556 * t561 * t8779 * t268;
    (t47591, t47593, t47595, t47601)
}
