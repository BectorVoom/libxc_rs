//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1336/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1336<F: Float>(t14657: F, t53233: F, t13808: F, t15278: F, t14733: F, t859: F, t892: F, t9914: F, t13793: F, t56320: F, t53245: F, t52996: F) -> (F, F, F, F, F, F) {
    let t57474 = t14657 * t53233;
    let t57476 = t13808 * t15278;
    let t57480 = t14733 * t859 * t892 * t9914;
    let t57482 = t56320 * t13793;
    let t57484 = t14657 * t53245;
    let t57486 = t14657 * t52996;
    (t57474, t57476, t57480, t57482, t57484, t57486)
}
