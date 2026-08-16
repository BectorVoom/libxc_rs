//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 620/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk620(t7701: f64, t7711: f64, t7713: f64, t7718: f64, t7721: f64, t7723: f64, t7725: f64, t7727: f64, t7734: f64, t7739: f64, t7743: f64, t7747: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8172 = 0.2993560425465952141e-1_f64 * t7701;
    let t8174 = 0.35922725105591425692e0_f64 * t7711;
    let t8175 = 0.17961362552795712846e0_f64 * t7713;
    let t8176 = 0.212822999466489197e-4_f64 * t7718;
    let t8177 = 0.1702583995731913576e-4_f64 * t7721;
    let t8178 = 0.5107751987195740728e-4_f64 * t7723;
    let t8179 = 0.5107751987195740728e-4_f64 * t7725;
    let t8180 = 0.1702583995731913576e-4_f64 * t7727;
    let t8181 = 0.638468998399467591e-4_f64 * t7734;
    let t8182 = 0.5107751987195740728e-4_f64 * t7739;
    let t8183 = 0.15323255961587222184e-3_f64 * t7743;
    let t8184 = 0.5107751987195740728e-4_f64 * t7747;
    (t8172, t8174, t8175, t8176, t8177, t8178, t8179, t8180, t8181, t8182, t8183, t8184)
}
