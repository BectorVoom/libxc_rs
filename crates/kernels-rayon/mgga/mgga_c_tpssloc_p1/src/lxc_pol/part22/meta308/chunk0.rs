//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1480/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1480(t14720: f64, t4775: f64, t699: f64, t11265: f64, t1661: f64, t11243: f64, t3270: f64, t4756: f64, t3287: f64, t4772: f64, t1657: f64, t3263: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14768 = 0.13418888888888888889e0_f64 * t14720;
    let t14781 = t699 * t4775;
    let t14782 = 0.22076e0_f64 * t14781;
    let t14801 = t11265 * t1661;
    let t14804 = t11243 * t1661;
    let t14808 = t3270 * t4756;
    let t14813 = t3287 * t4756;
    let t14818 = t699 * t4772;
    let t14838 = t1657 * t3263;
    (t14768, t14781, t14782, t14801, t14804, t14808, t14813, t14818, t14838)
}
