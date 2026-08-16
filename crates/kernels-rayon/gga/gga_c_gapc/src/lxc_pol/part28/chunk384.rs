//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 384/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk384(t1: f64, t102: f64, t1762: f64, t619: f64, t1686: f64, t185: f64, t505: f64, t567: f64, t1036: f64, t1689: f64, t147: f64, t19: f64, t995: f64) -> (f64, f64, f64, f64, f64) {
    let t1764 = t1762 * t1 * t102;
    let t1765 = t1764 * t619;
    let t1768 = t185 * t1686;
    let t1769 = t505 * t567;
    let t1771 = t1036 * t1689 * t1769;
    let t1775 = t995 * t19 * t147;
    (t1764, t1765, t1768, t1771, t1775)
}
