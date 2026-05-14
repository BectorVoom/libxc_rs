//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 660/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk660<F: Float>(t9958: F, t2463: F, t2868: F, t9965: F, t9967: F, t9972: F, t9976: F, t9978: F, t9980: F, t9982: F, t9986: F, t9991: F, t2471: F, t551: F, t739: F, t1734: F, t699: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10336 = 0.2993560425465952141e-1 * t9958;
    let t10337 = t2868 * t2463;
    let t10338 = 0.11974241701863808564e0 * t10337;
    let t10339 = 0.5107751987195740728e-4 * t9965;
    let t10340 = 0.5107751987195740728e-4 * t9967;
    let t10341 = 0.638468998399467591e-4 * t9972;
    let t10342 = 0.5107751987195740728e-4 * t9976;
    let t10343 = 0.5107751987195740728e-4 * t9978;
    let t10344 = 0.2993560425465952141e-1 * t9980;
    let t10345 = 0.1702583995731913576e-4 * t9982;
    let t10346 = 0.85129199786595678799e-5 * t9986;
    let t10348 = 0.212822999466489197e-4 * t9991;
    let t10350 = t2471 * t551;
    let t10351 = t739 * t10350;
    let t10352 = 0.11974241701863808564e0 * t10351;
    let t10353 = t699 * t1734;
    (t10336, t10338, t10339, t10340, t10341, t10342, t10343, t10344, t10345, t10346, t10348, t10350, t10352, t10353)
}
