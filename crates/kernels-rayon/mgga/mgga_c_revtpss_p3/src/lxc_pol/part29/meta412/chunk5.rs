//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1501/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1501(t140: f64, t3252: f64, t4574: f64, t1011: f64, t15145: f64, t4915: f64, t15149: f64, t15154: f64, t4919: f64, t15130: f64, t15135: f64, t1012: f64, t11821: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15993 = t140 * t3252;
    let t15994 = t15993 * t4574;
    let t15996 = t1011 * t15994 / 324.0_f64;
    let t15997 = t4915 * t15145;
    let t16000 = t4915 * t15149;
    let t16003 = t4919 * t15154;
    let t16006 = t4919 * t15130;
    let t16009 = t4919 * t15135;
    let t16012 = t1012 * t11821;
    (t15996, t15997, t16000, t16003, t16006, t16009, t16012)
}
