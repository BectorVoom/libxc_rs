//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1968/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1968(t102293: f64, t102296: f64, t102298: f64, t102306: f64, t102309: f64, t1444: f64, t1882: f64, t25921: f64, t25924: f64, t26333: f64, t26351: f64, t27837: f64, t28815: f64, t28840: f64, t543: f64, t7295: f64, t7301: f64, t96284: f64, t96287: f64, t96289: f64, t96292: f64, t96294: f64) -> f64 {
    let t102313 = 0.4336814094102599731e0_f64 * t7295 * t7301 * t26333 * t1882 * t543 - 0.52041769129231196772e1_f64 * t25921 * t28815 + 0.8673628188205199462e0_f64 * t27837 * t26351 - 0.34270468708064099208e-1_f64 * t102293 - 0.96373646535613327357e-2_f64 * t102296 - t96284 + 0.17135234354032049604e-1_f64 * t102298 - 0.52041769129231196772e1_f64 * t7295 * t25924 * t28840 * t1444 + t102306 - 0.45699670022203476294e-2_f64 * t96287 + t102309 + 0.34270468708064099208e-1_f64 * t96289 + 0.14456046980341999104e-1_f64 * t96292 - 0.25702851531048074406e-1_f64 * t96294;
    t102313
}
