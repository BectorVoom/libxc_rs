//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 694/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk694(t1268: f64, t2039: f64, t2314: f64, t5113: f64, t671: f64, t7040: f64, t7042: f64, t7056: f64, t2094: f64, t532: f64, t6879: f64, t6884: f64) -> (f64, f64, f64, f64) {
    let t7166 = 2.0_f64 * t1268 * t7056 + 2.0_f64 * t2039 * t2314 + 2.0_f64 * t2039 * t5113 + 2.0_f64 * t671 * t7042 + t7040;
    let t7170 = t532 * t2094;
    let t7171 = t7170 * t6879;
    let t7174 = 0.38381794893125283518e-1_f64 * t6884;
    (t7166, t7170, t7171, t7174)
}
