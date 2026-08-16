//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 480/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk480(t1052: f64, t369: f64, t361: f64, t351: f64, t1065: f64, t126: f64, t906: f64, t247: f64, t1063: f64, t1086: f64, t994: f64, t3090: f64) -> (f64, f64, f64, f64) {
    let t3104 = t1052 * t369;
    let t3105 = t361 * t3104;
    let t3106 = t351 * t3105;
    let t3109 = t126 * t1065;
    let t3110 = t3109 * t906;
    let t3111 = t247 * t3110;
    let t3112 = t1063 * t3111;
    let t3114 = t994 * t1086;
    let t3115 = t3114 * t3090;
    (t3106, t3109, t3112, t3115)
}
