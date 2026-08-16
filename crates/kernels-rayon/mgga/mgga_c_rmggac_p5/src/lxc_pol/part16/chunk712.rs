//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 712/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk712(t9976: f64, t9978: f64, t9980: f64, t9982: f64, t9986: f64, t9991: f64, t2471: f64, t551: f64, t739: f64, t1734: f64, t699: f64, t10000: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10342 = 0.5107751987195740728e-4_f64 * t9976;
    let t10343 = 0.5107751987195740728e-4_f64 * t9978;
    let t10344 = 0.2993560425465952141e-1_f64 * t9980;
    let t10345 = 0.1702583995731913576e-4_f64 * t9982;
    let t10346 = 0.85129199786595678799e-5_f64 * t9986;
    let t10348 = 0.212822999466489197e-4_f64 * t9991;
    let t10350 = t2471 * t551;
    let t10351 = t739 * t10350;
    let t10352 = 0.11974241701863808564e0_f64 * t10351;
    let t10353 = t699 * t1734;
    let t10354 = t739 * t10353;
    let t10355 = 0.59871208509319042821e-1_f64 * t10354;
    let t10361 = 0.8980681276397856423e-1_f64 * t10000;
    (t10342, t10343, t10344, t10345, t10346, t10348, t10350, t10352, t10353, t10355, t10361)
}
