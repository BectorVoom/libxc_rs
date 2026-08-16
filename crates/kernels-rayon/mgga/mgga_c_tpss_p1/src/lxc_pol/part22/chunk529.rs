//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 529/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk529(t2187: f64, t2190: f64, t2193: f64, t2197: f64, t2199: f64, t2202: f64) -> f64 {
    let t2204 = -0.57538888888888888889e0_f64 * t2187 + 0.11507777777777777778e1_f64 * t2190 + 0.40256666666666666667e0_f64 * t2193 + 0.366775e-1_f64 * t2197 + 0.73355e-1_f64 * t2199 + 0.137975e0_f64 * t2202;
    t2204
}
