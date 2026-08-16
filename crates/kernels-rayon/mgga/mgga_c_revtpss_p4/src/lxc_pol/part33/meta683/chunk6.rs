//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2246/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2246(t1219: f64, t30800: f64, t1241: f64, t21100: f64, t7616: f64, t1256: f64, t30789: f64, t104770: f64, t1230: f64, t1252: f64, t20802: f64, t21095: f64, t21300: f64, t21334: f64, t2138: f64, t26870: f64, t29040: f64, t29097: f64, t30815: f64, t484: f64, t5261: f64, t6619: f64, t8184: f64, t97177: f64, t97250: f64) -> f64 {
    let t112301 = t30800 * t1219;
    let t112307 = t1241 * t7616 * t21100;
    let t112322 = t30789 * t1256;
    let t112327 = -t104770 + 11.0_f64 / 324.0_f64 * t112301 - 0.42874018118069736972e-3_f64 * t26870 * t21300 + t97177 / 1296.0_f64 + 0.14481890564325777821e-1_f64 * t112307 * t1252 + 0.57165357490759649296e-3_f64 * t97250 * t6619 - 0.57165357490759649296e-3_f64 * t29040 * t21095 + 0.85748036236139473944e-3_f64 * t29097 * t20802 - 0.45732285992607719436e-2_f64 * t5261 * t8184 * t484 + 0.14481890564325777821e-1_f64 * t1230 * t30815 * t484 + 0.28582678745379824648e-3_f64 * t112322 + 0.42874018118069736972e-3_f64 * t21334 * t2138 * t484;
    t112327
}
