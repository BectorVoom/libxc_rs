//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2730/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2730(t12772: f64, t21160: f64, t3625: f64, t11249: f64, t6622: f64, t12832: f64, t20926: f64, t15904: f64, t17394: f64, t13127: f64, t3682: f64, t6667: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t70857 = t3625 * t12772 * t21160;
    let t70890 = t6622 * t11249;
    let t70914 = t12832 * t20926;
    let t70916 = t17394 * t15904;
    let t70917 = t13127 * t70916;
    let t70942 = t6667 * t3682;
    (t70857, t70890, t70914, t70916, t70917, t70942)
}
