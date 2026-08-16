//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 369/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk369(t1694: f64, t654: f64, t633: f64, t118: f64, t1393: f64, t129: f64, t1303: f64, t172: f64, t153: f64, t181: f64, t1695: f64, t185: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1703 = t654 * t1694;
    let t1704 = t633 * t1703;
    let t1707 = t1393 * t118;
    let t1708 = t1707 * t129;
    let t1709 = t172 * t1303;
    let t1710 = t153 * t1709;
    let t1711 = t181 * t1710;
    let t1714 = t185 * t1695;
    (t1703, t1704, t1707, t1708, t1709, t1711, t1714)
}
