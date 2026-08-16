//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 711/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk711(t9936: f64, t9939: f64, t530: f64, t9639: f64, t9949: f64, t9952: f64, t9958: f64, t2463: f64, t2868: f64, t9965: f64, t9967: f64, t9972: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10329 = 0.5107751987195740728e-4_f64 * t9936;
    let t10330 = 0.10215503974391481456e-3_f64 * t9939;
    let t10332 = t530 * t9639;
    let t10333 = 0.4726e1_f64 * t10332;
    let t10334 = 0.35922725105591425692e0_f64 * t9949;
    let t10335 = 0.11974241701863808564e0_f64 * t9952;
    let t10336 = 0.2993560425465952141e-1_f64 * t9958;
    let t10337 = t2868 * t2463;
    let t10338 = 0.11974241701863808564e0_f64 * t10337;
    let t10339 = 0.5107751987195740728e-4_f64 * t9965;
    let t10340 = 0.5107751987195740728e-4_f64 * t9967;
    let t10341 = 0.638468998399467591e-4_f64 * t9972;
    (t10329, t10330, t10333, t10334, t10335, t10336, t10338, t10339, t10340, t10341)
}
