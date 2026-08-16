//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 781/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk781(t8372: f64, t8375: f64, t8379: f64, t8385: f64, t8388: f64, t8391: f64, t8394: f64, t8397: f64, t8400: f64, t8418: f64, t8423: f64, t8438: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t38197 = 0.23948483403727617128e0_f64 * t8372;
    let t38198 = 0.35922725105591425692e0_f64 * t8375;
    let t38200 = 0.23948483403727617128e0_f64 * t8379;
    let t38203 = 0.23948483403727617128e0_f64 * t8385;
    let t38204 = 0.23948483403727617128e0_f64 * t8388;
    let t38205 = 0.23948483403727617128e0_f64 * t8391;
    let t38206 = 0.35922725105591425692e0_f64 * t8394;
    let t38210 = 0.47896966807455234256e0_f64 * t8397;
    let t38211 = 0.23948483403727617128e0_f64 * t8400;
    let t38212 = 0.17025839957319135759e-4_f64 * t8418;
    let t38213 = 0.85129199786595678796e-5_f64 * t8423;
    let t38217 = 0.85129199786595678796e-5_f64 * t8438;
    (t38197, t38198, t38200, t38203, t38204, t38205, t38206, t38210, t38211, t38212, t38213, t38217)
}
