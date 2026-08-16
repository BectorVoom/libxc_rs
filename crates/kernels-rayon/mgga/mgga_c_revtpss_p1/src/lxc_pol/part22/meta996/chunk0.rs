//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3385/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3385(t19256: f64, t41583: f64, t11385: f64, t19255: f64, t2918: f64, t2875: f64, t41499: f64, t41502: f64, t6109: f64, t4707: f64, t972: f64, t4711: f64, t52238: f64) -> (f64, f64, f64, f64, f64) {
    let t63589 = 0.1034520258385468006e4_f64 * t41583 * t19256;
    let t63592 = 0.51726012919273400301e3_f64 * t11385 * t19255 * t2918;
    let t63596 = 0.24955700379505800916e5_f64 * t41499 * t6109 * t41502 * t2875;
    let t63597 = t972 * t4707;
    let t63600 = 0.4155806185363551302e3_f64 * t52238 * t4711 * t63597;
    (t63589, t63592, t63596, t63597, t63600)
}
