//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1281/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1281<F: Float>(t14469: F, t53571: F, t11819: F, t51555: F, t53236: F, t14733: F, t34838: F, t353: F, t859: F, t14657: F, t52993: F, t13791: F, t3916: F) -> (F, F, F, F, F) {
    let t56309 = t53571 * t14469;
    let t56312 = t51555 * t53236 * t11819;
    let t56316 = t14733 * t859 * t353 * t34838;
    let t56318 = t14657 * t52993;
    let t56320 = t3916 * t13791;
    (t56309, t56312, t56316, t56318, t56320)
}
