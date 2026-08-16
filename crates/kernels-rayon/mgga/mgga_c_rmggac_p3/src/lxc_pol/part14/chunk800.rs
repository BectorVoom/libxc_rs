//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 800/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk800(t8347: f64, t8353: f64, t8359: f64, t8363: f64, t8369: f64, t8372: f64, t8375: f64, t8379: f64, t8385: f64, t8388: f64, t8391: f64, t8394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t38191 = 0.72042316457491791906e-3_f64 * t8347;
    let t38192 = 0.72042316457491791906e-3_f64 * t8353;
    let t38193 = 0.72042316457491791906e-3_f64 * t8359;
    let t38194 = 0.72042316457491791906e-3_f64 * t8363;
    let t38196 = 0.68186654135613354322e-2_f64 * t8369;
    let t38197 = 0.23948483403727617128e0_f64 * t8372;
    let t38198 = 0.35922725105591425692e0_f64 * t8375;
    let t38200 = 0.23948483403727617128e0_f64 * t8379;
    let t38203 = 0.23948483403727617128e0_f64 * t8385;
    let t38204 = 0.23948483403727617128e0_f64 * t8388;
    let t38205 = 0.23948483403727617128e0_f64 * t8391;
    let t38206 = 0.35922725105591425692e0_f64 * t8394;
    (t38191, t38192, t38193, t38194, t38196, t38197, t38198, t38200, t38203, t38204, t38205, t38206)
}
