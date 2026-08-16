//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1098/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1098(t11910: f64, t11942: f64, t11932: f64, t11938: f64, t11952: f64, t11955: f64, t11960: f64, t11963: f64, t9221: f64, t9223: f64, t9226: f64, t9228: f64) -> (f64, f64) {
    let t12046 = 0.27785333333333333334e0_f64 * t11910;
    let t12060 = 0.34431666666666666666e0_f64 * t11942;
    let t12064 = 0.45908888888888888888e0_f64 * t9221 + 0.11477222222222222222e0_f64 * t9223 - 0.34431666666666666666e0_f64 * t9226 - 0.17215833333333333333e0_f64 * t9228 + 0.46308888888888888889e-1_f64 * t11932 + 0.3529725e1_f64 * t11955 + 0.22954444444444444444e0_f64 * t11938 - t12060 + 0.516475e0_f64 * t11952 + 0.6311625e0_f64 * t11960 + 0.46308888888888888889e-1_f64 * t11963;
    (t12046, t12064)
}
