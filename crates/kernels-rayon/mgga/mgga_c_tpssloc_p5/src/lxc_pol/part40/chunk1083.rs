//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1083/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1083(t10595: f64, t5698: f64, t896: f64, t4362: f64, t4370: f64, t2798: f64, t5705: f64, t10599: f64, t4378: f64, t2815: f64, t10296: f64, t10542: f64, t10545: f64, t10556: f64, t13552: f64, t13566: f64, t13675: f64, t13679: f64, t17173: f64, t17180: f64, t17185: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17210 = t10595 * t5698;
    let t17211 = t17210 * t896;
    let t17213 = t4362 * t4370;
    let t17215 = t2798 * t5705;
    let t17216 = t17215 * t896;
    let t17218 = t10599 * t5698;
    let t17219 = t17218 * t896;
    let t17221 = t4378 * t4370;
    let t17223 = t2815 * t5705;
    let t17224 = t17223 * t896;
    let t17238 = 0.12077e1_f64 * t17173 - t13675 + 0.36793333333333333333e-1_f64 * t13552 + t13679 - 0.40256666666666666668e0_f64 * t13566 - 0.91983333333333333333e-1_f64 * t10296 - t10542 - t10545 - 0.20128333333333333333e0_f64 * t17180 + 0.60385e0_f64 * t17185 - 0.13418888888888888889e0_f64 * t10556;
    (t17211, t17213, t17216, t17219, t17221, t17224, t17238)
}
