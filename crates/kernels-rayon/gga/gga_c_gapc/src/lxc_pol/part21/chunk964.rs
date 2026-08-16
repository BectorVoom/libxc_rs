//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 964/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk964(t2763: f64, t3137: f64, t327: f64, t7191: f64, t11752: f64, t1453: f64, t2206: f64, t1: f64, t311: f64, t3383: f64, t8676: f64, t3756: f64, t869: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11755 = t3137 * t327 * t2763 * t7191;
    let t11756 = t11752 * t11755;
    let t11758 = t2206 * t1453;
    let t11759 = t11758 * t1;
    let t11760 = t311 * t11759;
    let t11761 = t8676 * t3383;
    let t11762 = t11760 * t11761;
    let t11764 = t869 * t3756;
    (t11755, t11756, t11759, t11761, t11762, t11764)
}
