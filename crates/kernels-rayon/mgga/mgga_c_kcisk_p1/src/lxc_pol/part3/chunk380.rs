//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 380/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk380(t1904: f64, t752: f64, t751: f64, t724: f64, t1689: f64, t196: f64, t140: f64, t299: f64, t728: f64, t1797: f64, t41: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1905 = t1904 * t752;
    let t1906 = t751 * t751;
    let t1907 = 1.0_f64 / t1906;
    let t1908 = t724 * t1907;
    let t1909 = t1689 * t196;
    let t1918 = 0.26531111111111111111e-1_f64 * t140 * t299 * t728;
    let t1919 = t41 * t1797;
    (t1905, t1906, t1907, t1908, t1909, t1918, t1919)
}
