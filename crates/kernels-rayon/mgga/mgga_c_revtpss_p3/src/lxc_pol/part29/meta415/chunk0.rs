//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1516/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1516(t1655: f64, t697: f64, t1011: f64, t372: f64, t4806: f64, t15702: f64, t15688: f64, t3299: f64, t1043: f64, t905: f64, t606: f64, t3155: f64) -> (f64, f64, f64, f64) {
    let t16219 = t697 * t1655;
    let t16220 = t1011 * t16219;
    let t16222 = t372 * t4806;
    let t16223 = t16222 * t15702;
    let t16226 = t3299 * t15688;
    let t16227 = t1043 * t905;
    let t16228 = t16227 * t606;
    let t16229 = t3155 * t16228;
    (t16220, t16223, t16226, t16229)
}
