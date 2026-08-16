//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2743/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2743(t17729: f64, t20922: f64, t44425: f64, t17396: f64, t17617: f64, t1222: f64, t6658: f64, t697: f64, t6662: f64, t12916: f64, t20801: f64, t5340: f64) -> (f64, f64, f64, f64, f64) {
    let t71908 = t17729 * t44425 * t20922;
    let t71920 = t17396 * t17617;
    let t71928 = t1222 * t697 * t6658;
    let t71931 = t1222 * t697 * t6662;
    let t71971 = t5340 * t12916 * t20801;
    (t71908, t71920, t71928, t71931, t71971)
}
