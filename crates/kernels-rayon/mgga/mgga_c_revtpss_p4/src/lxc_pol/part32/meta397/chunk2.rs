//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1375/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1375(t14866: f64, t14871: f64, t18411: f64, t18416: f64, t18420: f64, t18424: f64, t18428: f64, t18433: f64, t18437: f64, t18442: f64, t18446: f64, t18451: f64, t2745: f64, t4362: f64, t851: f64) -> f64 {
    let t18454 = 0.71456696863449561619e-5_f64 * t18411 - 0.14291339372689912324e-4_f64 * t18416 + 0.71456696863449561619e-5_f64 * t18420 + 0.25410001404642664113e-3_f64 * t18424 - 0.17149607247227894789e-2_f64 * t4362 * t18428 + 0.25410001404642664113e-4_f64 * t18433 + 0.42874018118069736972e-2_f64 * t851 * t18437 - 0.57165357490759649296e-4_f64 * t18442 + 0.85748036236139473944e-3_f64 * t2745 * t18446 - 0.45351183609335988442e-1_f64 * t14866 + 0.85748036236139473944e-3_f64 * t2745 * t18451 - t14871;
    t18454
}
