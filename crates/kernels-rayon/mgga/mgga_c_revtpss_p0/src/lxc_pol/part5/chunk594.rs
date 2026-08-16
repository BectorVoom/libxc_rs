//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 594/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk594(t3223: f64, t366: f64, t1054: f64, t1058: f64, t1014: f64, t2857: f64, t1010: f64, t614: f64, t1016: f64, t140: f64, t1011: f64, t271: f64, t905: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3224 = t3223 * t366;
    let t3234 = t1054 * t1058;
    let t3236 = t1014 * t2857;
    let t3241 = t614 * t1010;
    let t3244 = t140 * t1016;
    let t3245 = t1011 * t3244;
    let t3252 = 1.0_f64 / t271 / t905;
    (t3224, t3234, t3236, t3241, t3245, t3252)
}
