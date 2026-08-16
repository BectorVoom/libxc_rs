//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1870/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1870(t25224: f64, t6572: f64, t1880: f64, t13053: f64, t1528: f64, t1912: f64, t23235: f64, t23281: f64, t25200: f64, t25206: f64, t25209: f64, t25211: f64, t25214: f64, t25218: f64, t25220: f64, t25222: f64, t259: f64, t2713: f64, t7538: f64, t855: f64) -> (f64, f64) {
    let t25225 = t25224 * t6572;
    let t25226 = t1880 * t25225;
    let t25228 = 0.19190897446562641759e-1_f64 * t23235 + 2.0_f64 * t855 * t25200 - t2713 * t7538 - t23281 * t1528 + 0.41123351671205660912e-2_f64 * t25206 - t13053 * t1912 + 0.38381794893125283518e-1_f64 * t25209 + 0.19190897446562641759e-1_f64 * t25211 - 0.82246703342411321825e-2_f64 * t25214 - 0.82246703342411321825e-2_f64 * t25218 + t25220 * t259 + t25222 * t259 - 0.82246703342411321825e-2_f64 * t25226;
    (t25225, t25228)
}
