//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 369/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk369(t1431: f64, t1689: f64, t1688: f64, t190: f64, t644: f64, t640: f64, t633: f64, t198: f64, t442: f64, t457: f64, t6: f64, t1037: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1690 = t1689 * t1431;
    let t1691 = t1688 * t1690;
    let t1694 = t190 * t644;
    let t1695 = t640 * t1694;
    let t1696 = t633 * t1695;
    let t1697 = t442 * t198;
    let t1698 = t457 * t6;
    let t1699 = t1037 * t1698;
    (t1690, t1691, t1694, t1695, t1696, t1697, t1698, t1699)
}
