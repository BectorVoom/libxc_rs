//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 974/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk974(t10383: f64, t10439: f64, t10287: f64, t10289: f64, t10292: f64, t10303: f64, t10306: f64, t10309: f64, t1317: f64, t1976: f64, t1981: f64, t1982: f64, t2049: f64, t3418: f64, t3423: f64, t3486: f64, t578: f64, t619: f64, t7679: f64, t7682: f64, t7690: f64, t91: f64) -> (f64, f64) {
    let t10440 = t10383 + t10439;
    let t10443 = t10287 * t91 - 8.0_f64 * t10289 * t619 + 20.0_f64 * t10292 * t1982 - 120.0_f64 * t10303 * t7690 + 40.0_f64 * t10306 * t1981 + 20.0_f64 * t10309 * t1981 - 4.0_f64 * t10440 * t578 - 4.0_f64 * t1317 * t7679 - 8.0_f64 * t1976 * t3486 - 4.0_f64 * t2049 * t3418 + 40.0_f64 * t3423 * t7682;
    (t10440, t10443)
}
