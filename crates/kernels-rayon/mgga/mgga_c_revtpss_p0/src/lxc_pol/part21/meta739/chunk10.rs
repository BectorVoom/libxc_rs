//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2602/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2602(t14109: f64, t47603: f64, t9681: f64, t14268: f64, t3915: f64, t686: f64, t72: f64, t14293: f64, t9664: f64, t13739: f64, t1445: f64, t4071: f64, t47525: f64, t47527: f64, t47531: f64, t47534: f64, t47537: f64, t47540: f64, t47893: f64, t47899: f64, t47904: f64, t47907: f64, t47909: f64) -> f64 {
    let t47913 = t47603 * t14109 * t9681;
    let t47918 = t3915 * t14268 * t72 * t686;
    let t47920 = t14293 * t9664;
    let t47922 = 0.39029762157531132075e-1_f64 * t47525 - 0.58544643236296698113e-1_f64 * t47893 + 0.39512695097613069591e1_f64 * t4071 * t13739 + 0.78059524315062264151e-1_f64 * t47527 + 0.58544643236296698114e-1_f64 * t47531 - 0.39029762157531132075e-2_f64 * t47899 + 0.19514881078765566037e-2_f64 * t47534 + 0.32927245914677557992e-1_f64 * t47537 - 0.30356481678079769392e-1_f64 * t47904 + 0.19514881078765566037e-2_f64 * t47907 - 0.19756347548806534796e1_f64 * t47909 * t1445 - 0.17563392970889009433e0_f64 * t47913 + 0.16463622957338778996e-1_f64 * t47540 - 0.29272321618148349057e-1_f64 * t47918 - 0.46263278077393568556e-2_f64 * t47920;
    t47922
}
