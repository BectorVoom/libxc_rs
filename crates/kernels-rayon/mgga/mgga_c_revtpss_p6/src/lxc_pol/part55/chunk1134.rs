//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1134/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1134(t120138: f64, t120043: f64, t31831: f64, t32247: f64, t32283: f64, t32192: f64, t8583: f64, t8584: f64, t1413: f64, t246: f64, t31752: f64, t3999: f64, t843: f64, t8589: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t120139 = 0.3718732920905101082e-4_f64 * t120138;
    let t120149 = t31831 * t120043;
    let t120952 = t32247 * t32283;
    let t120956 = t8583 * t8584 * t32192;
    let t120962 = t1413 * t246;
    let t120967 = t31752 * t32192 * t1413;
    let t120975 = t8583 * t8589 * t3999 * t843;
    (t120139, t120149, t120952, t120956, t120962, t120967, t120975)
}
