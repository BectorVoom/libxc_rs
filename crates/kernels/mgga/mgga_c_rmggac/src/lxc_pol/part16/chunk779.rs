//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 779/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk779<F: Float>(t40121: F, t40259: F, t9343: F, t942: F, t40339: F, t40349: F, t40351: F, t40354: F, t40356: F, t40458: F, t40479: F, t40505: F, t40560: F, t40562: F, t40578: F, t275: F, t9677: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t43288 = 0.11918087970123395032e-3 * t40121;
    let t43338 = 0.36366215538993788974e-1 * t40259;
    let t43366 = 0.4726e1 * t942 * t9343;
    let t43385 = 0.11918087970123395032e-3 * t40339;
    let t43390 = 0.39726959900411316772e-4 * t40349;
    let t43391 = 0.11918087970123395032e-3 * t40351;
    let t43392 = 0.11918087970123395032e-3 * t40354;
    let t43393 = 0.39726959900411316772e-4 * t40356;
    let t43422 = 0.15965655602485078085e0 * t40458;
    let t43433 = 0.39726959900411316772e-4 * t40479;
    let t43440 = 0.39726959900411316772e-4 * t40505;
    let t43466 = 0.1489760996265424379e-3 * t40560;
    let t43467 = 0.1489760996265424379e-3 * t40562;
    let t43472 = 0.15965655602485078085e0 * t40578;
    let t43481 = 2.0 * t275 * t9677;
    (t43288, t43338, t43366, t43385, t43390, t43391, t43392, t43393, t43422, t43433, t43440, t43466, t43467, t43472, t43481)
}
