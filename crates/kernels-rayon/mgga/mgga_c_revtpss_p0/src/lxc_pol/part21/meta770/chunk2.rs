//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2728/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2728(t50186: f64, t10495: f64, t14978: f64, t14979: f64, t1580: f64, t2765: f64, t2770: f64, t39549: f64, t39550: f64, t39554: f64, t41008: f64, t4474: f64, t50155: f64, t50164: f64, t50166: f64, t50169: f64, t50174: f64, t50178: f64, t50184: f64, t865: f64, t886: f64) -> f64 {
    let t50187 = 0.39029762157531132076e-1_f64 * t50186;
    let t50190 = -0.11044544084478153697e-3_f64 * t50155 + 0.39512695097613069591e1_f64 * t865 * t2770 * t14978 * t886 + 0.98781737744032673976e-1_f64 * t50164 - 0.17073386770573548589e-1_f64 * t50166 - 0.32927245914677557992e-1_f64 * t50169 - 0.65854491829355115987e0_f64 * t41008 * t1580 + 0.16463622957338778996e-1_f64 * t50174 - 0.19637199382202157274e-3_f64 * t50178 - t39549 - 0.33133632253434461091e-3_f64 * t39550 - 0.19756347548806534796e1_f64 * t2765 * t14979 - t50184 + t50187 + t39554 + 0.39512695097613069591e1_f64 * t4474 * t10495;
    t50190
}
