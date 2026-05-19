//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 711/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk711<F: Float>(t9936: F, t9939: F, t530: F, t9639: F, t9949: F, t9952: F, t9958: F, t2463: F, t2868: F, t9965: F, t9967: F, t9972: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10329 = F::cast_from(0.5107751987195740728e-4_f64) * t9936;
    let t10330 = F::cast_from(0.10215503974391481456e-3_f64) * t9939;
    let t10332 = t530 * t9639;
    let t10333 = F::new(0.4726e1) * t10332;
    let t10334 = F::cast_from(0.35922725105591425692e0_f64) * t9949;
    let t10335 = F::cast_from(0.11974241701863808564e0_f64) * t9952;
    let t10336 = F::cast_from(0.2993560425465952141e-1_f64) * t9958;
    let t10337 = t2868 * t2463;
    let t10338 = F::cast_from(0.11974241701863808564e0_f64) * t10337;
    let t10339 = F::cast_from(0.5107751987195740728e-4_f64) * t9965;
    let t10340 = F::cast_from(0.5107751987195740728e-4_f64) * t9967;
    let t10341 = F::cast_from(0.638468998399467591e-4_f64) * t9972;
    (t10329, t10330, t10333, t10334, t10335, t10336, t10338, t10339, t10340, t10341)
}
