//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1086/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1086(t13597: f64, t762: f64, t1450: f64, t5778: f64, t2516: f64, t5571: f64, t5566: f64, t72: f64, t757: f64, t1320: f64, t5567: f64, t5569: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13599 = 0.11696447245269292414e1_f64 * t13597 * t762;
    let t13600 = t5778 * t1450;
    let t13611 = t5571 * t2516;
    let t13613 = t5566 * t72;
    let t13615 = 0.36622894612013090108e-3_f64 * t13613 * t757;
    let t13620 = 8.0_f64 * t1320 * t5567;
    let t13621 = t1320 * t5569;
    (t13599, t13600, t13611, t13615, t13620, t13621)
}
