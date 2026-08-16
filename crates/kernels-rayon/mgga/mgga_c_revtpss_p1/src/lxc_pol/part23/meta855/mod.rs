//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta855 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2743;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2744;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta855(t17729: f64, t20922: f64, t44425: f64, t17396: f64, t17617: f64, t1222: f64, t6658: f64, t697: f64, t6662: f64, t12916: f64, t20801: f64, t5340: f64, t20805: f64, t5331: f64, t12784: f64, t21090: f64, t20293: f64, t57484: f64, t17735: f64, t70646: f64, t17423: f64, t21014: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t71908, t71920, t71928, t71931, t71971) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2743(t17729, t20922, t44425, t17396, t17617, t1222, t6658, t697, t6662, t12916, t20801, t5340);
        let (t71974, t71976, t72000, t72002, t72005) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2744(t12916, t20805, t5331, t12784, t21090, t1222, t20293, t57484, t17735, t70646, t17423, t21014);
    (t71908, t71920, t71928, t71931, t71971, t71974, t71976, t72000, t72002, t72005)
}
