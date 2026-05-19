//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 781/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk781<F: Float>(t8372: F, t8375: F, t8379: F, t8385: F, t8388: F, t8391: F, t8394: F, t8397: F, t8400: F, t8418: F, t8423: F, t8438: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t38197 = F::cast_from(0.23948483403727617128e0_f64) * t8372;
    let t38198 = F::cast_from(0.35922725105591425692e0_f64) * t8375;
    let t38200 = F::cast_from(0.23948483403727617128e0_f64) * t8379;
    let t38203 = F::cast_from(0.23948483403727617128e0_f64) * t8385;
    let t38204 = F::cast_from(0.23948483403727617128e0_f64) * t8388;
    let t38205 = F::cast_from(0.23948483403727617128e0_f64) * t8391;
    let t38206 = F::cast_from(0.35922725105591425692e0_f64) * t8394;
    let t38210 = F::cast_from(0.47896966807455234256e0_f64) * t8397;
    let t38211 = F::cast_from(0.23948483403727617128e0_f64) * t8400;
    let t38212 = F::cast_from(0.17025839957319135759e-4_f64) * t8418;
    let t38213 = F::cast_from(0.85129199786595678796e-5_f64) * t8423;
    let t38217 = F::cast_from(0.85129199786595678796e-5_f64) * t8438;
    (t38197, t38198, t38200, t38203, t38204, t38205, t38206, t38210, t38211, t38212, t38213, t38217)
}
