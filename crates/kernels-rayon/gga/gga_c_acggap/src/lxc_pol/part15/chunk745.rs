//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 745/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk745(t7519: f64, t7539: f64, t7545: f64, t7549: f64, t7557: f64, t7601: f64, t7611: f64, t7631: f64, t7638: f64, t7640: f64, t7671: f64, t7673: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8185 = 0.12579236915841660827e-2_f64 * t7519;
    let t8190 = 0.62896184579208304138e-3_f64 * t7539;
    let t8192 = 0.31448092289604152069e-3_f64 * t7545;
    let t8193 = 0.41930789719472202758e-3_f64 * t7549;
    let t8195 = 0.62896184579208304138e-3_f64 * t7557;
    let t8205 = 0.13073958333333333333e0_f64 * t7601;
    let t8209 = 0.21437009059034868486e-3_f64 * t7611;
    let t8219 = 0.37737710747524982482e-2_f64 * t7631;
    let t8220 = 0.27953859812981468505e-2_f64 * t7638;
    let t8221 = 0.25724410870841842184e-2_f64 * t7640;
    let t8232 = 0.42874018118069736972e-3_f64 * t7671;
    let t8233 = 13.0_f64 / 144.0_f64 * t7673;
    (t8185, t8190, t8192, t8193, t8195, t8205, t8209, t8219, t8220, t8221, t8232, t8233)
}
