//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2746/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2746(t1209: f64, t1284: f64, t6695: f64, t20849: f64, t3754: f64, t3781: f64, t6564: f64, t20800: f64, t3302: f64, t13141: f64, t1811: f64, t460: f64) -> (f64, f64, f64, f64, f64) {
    let t72267 = t1209 * t1284 * t6695;
    let t72270 = t20849 * t3754;
    let t72326 = t6564 * t3781;
    let t72329 = t20800 * t3302;
    let t72343 = t460 * t13141 * t1811;
    (t72267, t72270, t72326, t72329, t72343)
}
