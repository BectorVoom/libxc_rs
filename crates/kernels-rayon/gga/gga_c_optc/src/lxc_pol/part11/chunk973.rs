//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 973/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk973(t17338: f64, t17342: f64, t17346: f64, t17350: f64, t17354: f64, t17358: f64, t17399: f64, t17401: f64, t17403: f64, t17406: f64, t17409: f64, t17412: f64, t17419: f64, t8832: f64) -> f64 {
    let t17833 = -t8832 - 0.103295e1_f64 * t17346 + 0.309885e1_f64 * t17354 - 0.52945875e1_f64 * t17399 + 0.94674375e0_f64 * t17401 + 0.6311625e0_f64 * t17403 + 0.20839e0_f64 * t17406 - 0.62517e0_f64 * t17409 - 0.46308888888888888889e-1_f64 * t17412 - 0.57386111111111111112e0_f64 * t17338 + 0.20659e1_f64 * t17342 - 0.309885e1_f64 * t17350 - 0.516475e0_f64 * t17358 - 0.104195e0_f64 * t17419;
    t17833
}
