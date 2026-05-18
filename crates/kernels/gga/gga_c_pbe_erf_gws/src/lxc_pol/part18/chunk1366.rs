//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1366/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1366<F: Float>(t13793: F, t56320: F, t14657: F, t53245: F, t52996: F, t14469: F, t53229: F, t13888: F, t2408: F, t54464: F, t57449: F, t57454: F, t57458: F, t57462: F, t57468: F, t57472: F, t57474: F, t57476: F, t57480: F, t9283: F, t9926: F) -> F {
    let t57482 = t56320 * t13793;
    let t57484 = t14657 * t53245;
    let t57486 = t14657 * t52996;
    let t57488 = t53229 * t14469;
    let t57490 = t57449 / F::new(96.0) - t57454 / F::new(1536.0) + t57458 / F::new(96.0) + t57462 / F::new(3072.0) - t2408 * t9283 * t13888 * t9926 / F::new(12.0) + t57468 / F::new(96.0) - F::new(7.0) / F::new(288.0) * t57472 - t54464 - t57474 / F::new(48.0) - F::new(7.0) / F::new(2304.0) * t57476 - t57480 / F::new(96.0) - t57482 / F::new(48.0) - t57484 / F::new(24.0) - t57486 / F::new(24.0) + F::new(7.0) / F::new(72.0) * t57488;
    t57490
}
