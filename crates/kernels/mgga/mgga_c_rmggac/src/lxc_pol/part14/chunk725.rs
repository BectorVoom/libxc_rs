//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 725/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk725<F: Float>(t8372: F, t8375: F, t8379: F, t8385: F, t8388: F, t8391: F, t8394: F, t7267: F, t7270: F, t7277: F, t7280: F, t7286: F, t8040: F, t8397: F, t8400: F, t8418: F) -> (F, F, F, F, F, F) {
    let t38197 = 0.23948483403727617128e0 * t8372;
    let t38198 = 0.35922725105591425692e0 * t8375;
    let t38200 = 0.23948483403727617128e0 * t8379;
    let t38203 = 0.23948483403727617128e0 * t8385;
    let t38204 = 0.23948483403727617128e0 * t8388;
    let t38205 = 0.23948483403727617128e0 * t8391;
    let t38206 = 0.35922725105591425692e0 * t8394;
    let t38207 = t38200 + t7267 + 0.36366215538993788972e-1 * t7270 + t7277 + 0.14546486215597515589e0 * t7280 + t7286 - t8040 + t38203 - t38204 - t38205 + t38206;
    let t38210 = 0.47896966807455234256e0 * t8397;
    let t38211 = 0.23948483403727617128e0 * t8400;
    let t38212 = 0.17025839957319135759e-4 * t8418;
    (t38197, t38198, t38207, t38210, t38211, t38212)
}
