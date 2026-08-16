//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3734/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3734(t19666: f64, t5405: f64, t12832: f64, t20926: f64, t15904: f64, t17394: f64, t13127: f64, t1248: f64, t1469: f64, t606: f64, t3682: f64, t6667: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t70910 = t19666 * t5405;
    let t70914 = t12832 * t20926;
    let t70916 = t17394 * t15904;
    let t70917 = t13127 * t70916;
    let t70932 = t1469 * t1248;
    let t70933 = t70932 * t606;
    let t70942 = t6667 * t3682;
    (t70910, t70914, t70916, t70917, t70932, t70933, t70942)
}
