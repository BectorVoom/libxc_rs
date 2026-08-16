//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1820/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1820(t1017: f64, t1028: f64, t1047: f64, t25490: f64, t25495: f64, t25498: f64, t25500: f64, t25505: f64, t25509: f64, t25512: f64, t25517: f64, t25522: f64, t25526: f64, t25529: f64, t25532: f64, t25535: f64, t25538: f64, t25539: f64, t3097: f64, t3130: f64, t3136: f64, t3157: f64, t3164: f64, t3208: f64, t3220: f64, t348: f64, t7117: f64, t7122: f64) -> f64 {
    let t25542 = -0.85748036236139473944e-3_f64 * t25490 * t1028 - 0.42874018118069736972e-3_f64 * t7117 * t3220 + 0.45732285992607719436e-2_f64 * t25495 * t1028 - 0.57165357490759649296e-3_f64 * t25498 + 0.85748036236139473944e-3_f64 * t25500 * t3208 + 0.85748036236139473944e-3_f64 * t25505 * t3157 - 0.42874018118069736972e-3_f64 * t25509 * t3164 + 0.85748036236139473944e-3_f64 * t25512 * t1047 + 0.57165357490759649296e-3_f64 * t25517 * t3097 + 0.42874018118069736972e-3_f64 * t7122 * t3136 - 0.57165357490759649296e-3_f64 * t25522 * t3130 - 0.45732285992607719436e-2_f64 * t25526 * t1047 + 0.57165357490759649296e-3_f64 * t25529 + 11.0_f64 / 108.0_f64 * t25532 * t348 - t25535 / 54.0_f64 - t25538 - t25539 * t1017 / 54.0_f64;
    t25542
}
