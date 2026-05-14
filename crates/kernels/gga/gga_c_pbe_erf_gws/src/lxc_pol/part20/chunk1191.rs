//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1191/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1191<F: Float>(t14657: F, t53233: F, t13808: F, t15278: F, t14733: F, t859: F, t892: F, t9914: F, t13793: F, t56320: F, t53245: F, t52996: F, t14469: F, t53229: F, t13888: F, t2408: F, t54464: F, t57449: F, t57454: F, t57458: F, t57462: F, t57468: F, t57472: F, t9283: F, t9926: F) -> (F,) {
    let t57474 = t14657 * t53233;
    let t57476 = t13808 * t15278;
    let t57480 = t14733 * t859 * t892 * t9914;
    let t57482 = t56320 * t13793;
    let t57484 = t14657 * t53245;
    let t57486 = t14657 * t52996;
    let t57488 = t53229 * t14469;
    let t57490 = t57449 / 96.0 - t57454 / 1536.0 + t57458 / 96.0 + t57462 / 3072.0 - t2408 * t9283 * t13888 * t9926 / 12.0 + t57468 / 96.0 - 7.0 / 288.0 * t57472 - t54464 - t57474 / 48.0 - 7.0 / 2304.0 * t57476 - t57480 / 96.0 - t57482 / 48.0 - t57484 / 24.0 - t57486 / 24.0 + 7.0 / 72.0 * t57488;
    (t57490,)
}
