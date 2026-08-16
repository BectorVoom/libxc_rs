//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2614/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2614(t13872: f64, t221: f64, t3978: f64, t9921: f64, t1320: f64, t13632: f64, t13672: f64, t1317: f64, t13680: f64, t3860: f64, t5567: f64, t46960: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48141 = t221 * t13872;
    let t48143 = t3978 * t9921 * t48141;
    let t48152 = t1320 * t13632;
    let t48153 = 12.0_f64 * t48152;
    let t48154 = t1320 * t13672;
    let t48155 = 12.0_f64 * t48154;
    let t48157 = 24.0_f64 * t1317 * t13680;
    let t48158 = t3860 * t5567;
    let t48159 = 36.0_f64 * t48158;
    let t48160 = 36.0_f64 * t46960;
    (t48143, t48153, t48155, t48157, t48159, t48160)
}
