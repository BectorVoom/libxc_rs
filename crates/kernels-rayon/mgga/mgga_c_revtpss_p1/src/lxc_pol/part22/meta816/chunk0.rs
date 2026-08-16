//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2925/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2925(t4146: f64, t1455: f64, t5808: f64, t1892: f64, t9646: f64, t9648: f64, t1904: f64, t47567: f64, t14110: f64, t47530: f64, t1427: f64, t1903: f64, t22: f64, t9647: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47671 = t4146 * t4146;
    let t47672 = 1.0_f64 / t47671;
    let t47730 = t1455 * t5808;
    let t47764 = t9646 * t1892 * t9648;
    let t47772 = t47567 * t1904;
    let t47777 = t47530 * t14110;
    let t47781 = t9647 * t1427 * t1903 * t22;
    (t47672, t47730, t47764, t47772, t47777, t47781)
}
