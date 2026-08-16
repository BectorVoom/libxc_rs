//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2044/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2044(t3268: f64, t7143: f64, t3057: f64, t25460: f64, t25698: f64, t1035: f64, t25586: f64, t93484: f64, t994: f64, t1071: f64, t7150: f64, t8521: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93920 = t7143 * t3268;
    let t93921 = t3057 * t93920;
    let t93928 = t25698 * t25460;
    let t93939 = t1035 * t25586;
    let t93959 = t994 * t93484;
    let t93962 = t7150 * t1071;
    let t93963 = t93962 * t8521;
    (t93920, t93921, t93928, t93939, t93959, t93963)
}
