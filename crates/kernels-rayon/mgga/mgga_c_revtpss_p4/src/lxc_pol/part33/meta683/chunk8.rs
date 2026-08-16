//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2248/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2248(t30799: f64, t800: f64, t21270: f64, t2137: f64, t467: f64, t20926: f64, t26870: f64, t104647: f64, t104752: f64, t104844: f64, t104924: f64, t1227: f64, t1266: f64, t1791: f64, t17934: f64, t20838: f64, t20923: f64, t20934: f64, t29062: f64, t29096: f64, t29100: f64, t5279: f64, t5320: f64, t5343: f64, t6611: f64, t97174: f64, t97292: f64) -> f64 {
    let t112350 = t30799 * t800;
    let t112356 = t467 * t2137 * t21270;
    let t112364 = t26870 * t20926;
    let t112372 = 0.57165357490759649296e-3_f64 * t104752 * t5279 + t104844 - 0.95275595817932748827e-3_f64 * t104647 * t20923 - 11.0_f64 / 324.0_f64 * t112350 * t1227 + 0.57165357490759649296e-3_f64 * t97174 * t20934 - 0.96545937095505185473e-2_f64 * t112356 * t1266 - 0.85748036236139473944e-3_f64 * t29100 * t20838 + 0.17149607247227894789e-2_f64 * t17934 * t29096 * t5343 - 0.57165357490759649296e-3_f64 * t112364 + 0.85748036236139473944e-3_f64 * t97292 * t6611 + 0.45732285992607719436e-2_f64 * t104924 * t1791 + 0.45732285992607719436e-2_f64 * t29062 * t5320;
    t112372
}
