//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 886/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk886(t15925: f64, t3090: f64, t15822: f64, t3160: f64, t1655: f64, t697: f64, t1011: f64, t1678: f64, t3057: f64, t3286: f64, t4746: f64, t1647: f64, t3298: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15926 = t15925 * t3090;
    let t15932 = t15822 * t3160;
    let t16219 = t697 * t1655;
    let t16220 = t1011 * t16219;
    let t16284 = t3057 * t1678;
    let t16502 = t4746 * t3286;
    let t16509 = t1647 * t3298;
    (t15926, t15932, t16220, t16284, t16502, t16509)
}
