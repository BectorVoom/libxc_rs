//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2238/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2238(t17361: f64, t7618: f64, t17289: f64, t2138: f64, t1238: f64, t16729: f64, t17461: f64, t17536: f64, t17662: f64, t26880: f64, t29047: f64, t29054: f64, t29086: f64, t3663: f64, t97174: f64, t97179: f64, t97220: f64, t97222: f64, t97239: f64, t97247: f64) -> f64 {
    let t104905 = t7618 * t17361;
    let t104916 = t17289 * t2138;
    let t104921 = 0.57165357490759649296e-3_f64 * t26880 * t17536 - t97220 / 864.0_f64 - t97222 / 432.0_f64 - 0.95275595817932748827e-4_f64 * t104905 + 0.57165357490759649296e-3_f64 * t97174 * t17662 + 0.17149607247227894789e-2_f64 * t97179 * t17461 - 0.3811023832717309953e-3_f64 * t97239 + t29047 * t29054 * t16729 / 216.0_f64 - 0.19055119163586549765e-3_f64 * t97247 - 0.85748036236139473944e-3_f64 * t104916 * t1238 - 0.42874018118069736972e-3_f64 * t29086 * t3663;
    t104921
}
