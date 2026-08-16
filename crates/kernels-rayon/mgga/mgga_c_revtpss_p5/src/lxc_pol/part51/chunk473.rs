//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 473/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk473(t1024: f64, t1053: f64, t1026: f64, t127: f64, t371: f64, t1025: f64, t225: f64, t3046: f64, t366: f64, t1054: f64, t1058: f64, t1010: f64, t614: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3211 = t1024 * t1053;
    let t3215 = t371 * t127 * t1026;
    let t3216 = t1025 * t3215;
    let t3223 = t3046 * t225;
    let t3224 = t3223 * t366;
    let t3234 = t1054 * t1058;
    let t3241 = t614 * t1010;
    (t3211, t3215, t3216, t3223, t3224, t3234, t3241)
}
