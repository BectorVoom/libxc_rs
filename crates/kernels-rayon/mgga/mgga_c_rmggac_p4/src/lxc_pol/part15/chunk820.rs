//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 820/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk820(t40259: f64, t16156: f64, t9213: f64, t1965: f64, t1967: f64, t28: f64, t8511: f64, t1562: f64, t7399: f64, t118: f64, t1986: f64, t352: f64, t39866: f64) -> (f64, f64, f64, f64, f64) {
    let t40260 = 0.18183107769496894486e-1_f64 * t40259;
    let t40262 = t16156 * t9213;
    let t40263 = 0.39726959900411316772e-4_f64 * t40262;
    let t40278 = t8511 * t1965 * t1967 * t28;
    let t40294 = 0.4726e1_f64 * t1562 * t7399;
    let t40313 = t1986 * t118 * t39866 * t352;
    (t40260, t40263, t40278, t40294, t40313)
}
