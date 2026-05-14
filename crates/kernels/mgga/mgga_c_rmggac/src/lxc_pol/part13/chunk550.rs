//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 550/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk550<F: Float>(t7600: F, t7603: F, t7586: F, t793: F, t7590: F, t797: F, t851: F, t854: F, t36: F, t839: F, t3814: F, t265: F, t333: F, t7596: F, t3810: F, t305: F, t830: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7604 = t7603 * t7600;
    let t7606 = t793 * t7586;
    let t7608 = t797 * t7590;
    let t7610 = t851 * t7586;
    let t7612 = t854 * t7590;
    let t7614 = t36 * t839;
    let t7615 = t3814 * t7614;
    let t7617 = t265 * t333;
    let t7618 = t797 * t7617;
    let t7620 = t851 * t7596;
    let t7622 = t3810 * t7614;
    let t7625 = t854 * t7617;
    let t7627 = t305 * t830;
    (t7604, t7606, t7608, t7610, t7612, t7614, t7615, t7617, t7618, t7620, t7622, t7625, t7627)
}
