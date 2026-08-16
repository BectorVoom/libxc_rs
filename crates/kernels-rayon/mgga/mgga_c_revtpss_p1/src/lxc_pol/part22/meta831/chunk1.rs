//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2953/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2953(t2661: f64, t3938: f64, t3992: f64, t48533: f64, t14045: f64, t9810: f64, t13774: f64, t1399: f64, t13927: f64, t48100: f64, t9816: f64, t13910: f64) -> (f64, f64, f64, f64, f64) {
    let t48536 = t2661 * t3992 * t48533 * t3938;
    let t48540 = t2661 * t3992 * t14045 * t9810;
    let t48544 = t2661 * t3992 * t13774 * t1399;
    let t48548 = t9816 * t48100 * t13927;
    let t48553 = t2661 * t3992 * t13910 * t1399;
    (t48536, t48540, t48544, t48548, t48553)
}
