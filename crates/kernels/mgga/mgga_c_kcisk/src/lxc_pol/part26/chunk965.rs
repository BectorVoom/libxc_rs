//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 965/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk965<F: Float>(t13526: F, t13616: F, t13666: F, t20292: F, t20373: F, t26110: F, t26113: F, t26116: F, t26119: F, t26122: F, t26126: F, t26138: F, t26150: F, t26156: F, t26159: F, t26162: F, t26165: F, t26168: F, t26185: F, t26195: F, t26198: F, t26219: F) -> (F,) {
    let t26221 = -0.18257037037037037037e0 * t13616 - 0.36514074074074074073e0 * t20373 - 0.26574814814814814815e0 * t20292 + 0.32862666666666666666e0 * t26110 - 0.73028148148148148146e-1 * t26113 - 0.21908444444444444444e0 * t26116 - 0.98587999999999999998e0 * t26119 + 0.13145066666666666666e1 * t26122 - 0.13287407407407407408e0 * t13526 + 0.32862666666666666666e0 * t26126 + t26185 + 0.23917333333333333334e1 * t26156 - 0.19931111111111111111e0 * t26162 + 0.59793333333333333334e0 * t26165 - 0.19931111111111111111e0 * t26150 + 0.99655555555555555557e-1 * t26159 - 0.29896666666666666667e0 * t26168 + 0.66437037037037037037e-1 * t26138 - 0.16431333333333333333e0 * t26195 + 0.10954222222222222222e0 * t26198 - t13666 + t26219;
    (t26221,)
}
