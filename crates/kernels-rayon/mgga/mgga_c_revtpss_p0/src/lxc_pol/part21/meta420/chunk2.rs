//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1909/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1909(t13581: f64, t187: f64, t1857: f64, t3857: f64, t5591: f64, t566: f64, t9375: f64, t177: f64, t5566: f64, t762: f64, t1450: f64, t5778: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13583 = 0.19751673498613801407e-1_f64 * t13581 * t187;
    let t13584 = t3857 * t1857;
    let t13585 = 20.0_f64 * t13584;
    let t13586 = t566 * t5591;
    let t13593 = 0.11696447245269292414e1_f64 * t9375;
    let t13597 = t5566 * t177;
    let t13599 = 0.11696447245269292414e1_f64 * t13597 * t762;
    let t13600 = t5778 * t1450;
    (t13583, t13585, t13586, t13593, t13597, t13599, t13600)
}
