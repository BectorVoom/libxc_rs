//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2720/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2720(t21063: f64, t3678: f64, t17225: f64, t5381: f64, t1261: f64, t20791: f64, t3172: f64, t13058: f64, t20786: f64, t11262: f64, t3711: f64, t6618: f64) -> (f64, f64, f64, f64, f64) {
    let t70265 = t21063 * t3678;
    let t70270 = t5381 * t17225;
    let t70273 = t1261 * t3172 * t20791;
    let t70275 = t13058 * t20786;
    let t70278 = t3711 * t11262 * t6618;
    (t70265, t70270, t70273, t70275, t70278)
}
