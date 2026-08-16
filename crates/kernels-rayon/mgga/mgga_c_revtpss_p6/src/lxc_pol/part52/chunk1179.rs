//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1179/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1179(t126030: f64, t25207: f64, t27363: f64, t30: f64, t119763: f64, t1561: f64, t1558: f64, t257: f64, t119767: f64, t247: f64, t2749: f64, t119757: f64, t31846: f64, t4451: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t126031 = t25207 * t126030;
    let t126037 = t30 * t27363;
    let t126043 = t119763 * t1561;
    let t126046 = t257 * t1558;
    let t126049 = t119767 * t247 * t126046 * t2749;
    let t126052 = t31846 * t119757 * t4451;
    (t126031, t126037, t126043, t126046, t126049, t126052)
}
