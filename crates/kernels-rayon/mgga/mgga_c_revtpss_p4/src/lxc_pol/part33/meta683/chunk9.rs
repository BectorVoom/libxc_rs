//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2249/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2249(t20850: f64, t2138: f64, t29086: f64, t5362: f64, t104703: f64, t104863: f64, t104872: f64, t104916: f64, t104946: f64, t112220: f64, t1238: f64, t1791: f64, t20858: f64, t20952: f64, t21042: f64, t21310: f64, t26870: f64, t29097: f64, t3767: f64, t5320: f64, t5343: f64, t5354: f64, t97179: f64) -> f64 {
    let t112373 = t20850 * t2138;
    let t112380 = t29086 * t5362;
    let t112395 = -0.42874018118069736972e-3_f64 * t112373 * t1238 - 0.85748036236139473944e-3_f64 * t104916 * t1791 - 0.85748036236139473944e-3_f64 * t29086 * t5320 - 0.57165357490759649296e-3_f64 * t112380 + 0.85748036236139473944e-3_f64 * t97179 * t20858 - t104863 + 0.17149607247227894789e-2_f64 * t29097 * t20952 - 0.42874018118069736972e-3_f64 * t26870 * t21042 - 0.11433071498151929859e-2_f64 * t104946 * t21310 - 0.85748036236139473944e-3_f64 * t104703 * t5354 - 0.91464571985215438872e-2_f64 * t3767 * t112220 * t5343 + t104872;
    t112395
}
