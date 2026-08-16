//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 484/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk484(t3201: f64, t367: f64, t225: f64, t3057: f64, t1024: f64, t1053: f64, t1026: f64, t127: f64, t371: f64, t1025: f64, t3046: f64, t366: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3203 = 0.47637797908966374413e-4_f64 * t367 * t3201;
    let t3204 = t3057 * t225;
    let t3211 = t1024 * t1053;
    let t3215 = t371 * t127 * t1026;
    let t3216 = t1025 * t3215;
    let t3223 = t3046 * t225;
    let t3224 = t3223 * t366;
    (t3203, t3204, t3211, t3216, t3223, t3224)
}
