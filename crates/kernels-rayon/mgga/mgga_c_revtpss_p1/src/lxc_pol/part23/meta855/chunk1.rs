//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2744/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2744(t12916: f64, t20805: f64, t5331: f64, t12784: f64, t21090: f64, t1222: f64, t20293: f64, t57484: f64, t17735: f64, t70646: f64, t17423: f64, t21014: f64) -> (f64, f64, f64, f64, f64) {
    let t71974 = t5331 * t12916 * t20805;
    let t71976 = t12784 * t21090;
    let t72000 = t1222 * t57484 * t20293;
    let t72002 = t17735 * t70646;
    let t72005 = t21014 * t17423;
    (t71974, t71976, t72000, t72002, t72005)
}
