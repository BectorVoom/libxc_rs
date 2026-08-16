//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 779/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk779(t1020: f64, t1053: f64, t1021: f64, t1058: f64, t225: f64, t3043: f64, t366: f64, t371: f64, t373: f64, t676: f64, t367: f64, t3057: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3191 = t1020 * t1053;
    let t3194 = t1021 * t1058;
    let t3196 = t3043 * t225;
    let t3197 = t3196 * t366;
    let t3201 = t371 * t676 * t373;
    let t3203 = 0.47637797908966374413e-4_f64 * t367 * t3201;
    let t3204 = t3057 * t225;
    (t3191, t3194, t3196, t3197, t3201, t3203, t3204)
}
