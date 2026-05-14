//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 74/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk74<F: Float>(t210: F, t214: F, t208: F, t195: F, t143: F, rho0: F, rho1: F) -> (F, F, F, F, F, F, F) {
    let t215 = t210 * t214;
    let t217 = 1.0 + t208 / 8.0 - t215 / 64.0;
    let t218 = 1.0 / t217;
    let t219 = t195 * t218;
    let t220 = rho0 - rho1;
    let t221 = t220 * t143;
    let t222 = 1.0 + t221;
    (t215, t217, t218, t219, t220, t221, t222)
}
