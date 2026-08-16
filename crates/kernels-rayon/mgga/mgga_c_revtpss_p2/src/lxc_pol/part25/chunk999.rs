//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 999/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk999(t11876: f64, t3117: f64, t1016: f64, t697: f64, t1011: f64, t1010: f64, t2270: f64, t3241: f64, t3244: f64, t1058: f64, t3197: f64, t11132: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11877 = t3117 * t11876;
    let t11880 = t697 * t1016;
    let t11881 = t1011 * t11880;
    let t11883 = t2270 * t1010;
    let t11886 = t3241 * t3244;
    let t11888 = t3197 * t1058;
    let t11890 = 0.25925925925925925926e-1_f64 * t11132;
    (t11877, t11880, t11881, t11883, t11886, t11888, t11890)
}
