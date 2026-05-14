//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 528/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk528<F: Float>(t515: F, t7448: F, t1971: F, t1970: F, t1969: F, t7229: F) -> (F, F, F) {
    let t7449 = t515 * t7448;
    let t7450 = t1971 * t7449;
    let t7451 = t1970 * t7450;
    let t7452 = 0.85129199786595678796e-5 * t7451;
    let t7453 = t7229 * t1969;
    (t7450, t7452, t7453)
}
