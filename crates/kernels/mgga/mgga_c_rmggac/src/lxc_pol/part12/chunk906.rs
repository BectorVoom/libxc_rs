//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 906/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk906<F: Float>(t2392: F, t262: F, t794: F, t34738: F, t321: F, t8915: F, t7204: F, t333: F, t8700: F, t8630: F, t352: F, t7192: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t39662 = t262 * t2392 * t794;
    let t39663 = t34738 * t39662;
    let t39665 = t8915 * t321;
    let t39666 = t262 * t39665;
    let t39667 = t7204 * t39666;
    let t39670 = t8700 * t333;
    let t39671 = t262 * t39670;
    let t39672 = t8630 * t39671;
    let t39674 = t8700 * t352;
    let t39675 = t262 * t39674;
    let t39676 = t7192 * t39675;
    (t39662, t39663, t39665, t39666, t39667, t39670, t39671, t39672, t39674, t39675, t39676)
}
