//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1102/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1102(t11910: f64, t11942: f64, t11932: f64, t11938: f64, t11952: f64, t11955: f64, t11960: f64, t11963: f64, t9221: f64, t9223: f64, t9226: f64, t9228: f64) -> (f64, f64) {
    let t12115 = 0.22076e0_f64 * t11910;
    let t12129 = 0.20128333333333333334e0_f64 * t11942;
    let t12133 = 0.26837777777777777778e0_f64 * t9221 + 0.67094444444444444447e-1_f64 * t9223 - 0.20128333333333333334e0_f64 * t9226 - 0.10064166666666666667e0_f64 * t9228 + 0.36793333333333333334e-1_f64 * t11932 + 0.258925e1_f64 * t11955 + 0.13418888888888888889e0_f64 * t11938 - t12129 + 0.301925e0_f64 * t11952 + 0.16504875e0_f64 * t11960 + 0.36793333333333333333e-1_f64 * t11963;
    (t12115, t12133)
}
