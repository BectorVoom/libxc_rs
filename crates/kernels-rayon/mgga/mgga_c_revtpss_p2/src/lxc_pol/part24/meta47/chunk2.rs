//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 326/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk326(t418: f64, t408: f64, t409: f64, t1118: f64, t406: f64, t281: f64, t414: f64, t926: f64, t240: f64, t462: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1129 = t418 * t418;
    let t1130 = 1.0_f64 / t1129;
    let t1131 = t408 * t1130;
    let t1132 = 1.0_f64 / t409;
    let t1137 = 0.29896666666666666667e0_f64 * t1118;
    let t1139 = f64::sqrt(t406);
    let t1143 = t281 * t926 * t414;
    let t1144 = 0.82156666666666666667e-1_f64 * t1143;
    let t1145 = t240 * t462;
    (t1129, t1130, t1131, t1132, t1137, t1139, t1143, t1144, t1145)
}
