//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 968/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk968<F: Float>(t13526: F, t13616: F, t13746: F, t20292: F, t20373: F, t26110: F, t26113: F, t26116: F, t26119: F, t26122: F, t26126: F, t26138: F, t26150: F, t26156: F, t26159: F, t26162: F, t26165: F, t26168: F, t26195: F, t26198: F, t26267: F, t26287: F) -> (F,) {
    let t26289 = -0.18396666666666666667e0 * t13616 - 0.36793333333333333333e0 * t20373 - 0.26837777777777777779e0 * t20292 + 0.33114e0 * t26110 - 0.73586666666666666666e-1 * t26113 - 0.22076e0 * t26116 - 0.99342e0 * t26119 + 0.132456e1 * t26122 - 0.13418888888888888889e0 * t13526 + 0.33114e0 * t26126 + t26267 + 0.24154e1 * t26156 - 0.20128333333333333333e0 * t26162 + 0.60385e0 * t26165 - 0.20128333333333333333e0 * t26150 + 0.10064166666666666667e0 * t26159 - 0.301925e0 * t26168 + 0.67094444444444444443e-1 * t26138 - 0.16557e0 * t26195 + 0.11038e0 * t26198 - t13746 + t26287;
    (t26289,)
}
