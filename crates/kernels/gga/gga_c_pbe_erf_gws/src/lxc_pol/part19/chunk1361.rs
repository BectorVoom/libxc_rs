//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1361/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1361<F: Float>(t1115: F, t11342: F, t1205: F, t12213: F, t14311: F, t14911: F, t15025: F, t2376: F, t2408: F, t2409: F, t3040: F, t3066: F, t3067: F, t3306: F, t3913: F, t4083: F, t4227: F, t52251: F, t55114: F, t55117: F, t55145: F, t55764: F, t56362: F, t56366: F, t56374: F, t56404: F, t56431: F, t9807: F) -> F {
    let t58140 = -t1115 * t55764 / F::new(48.0) - t3040 * t14911 / F::new(48.0) - t3913 * t14311 / F::new(96.0) + t56362 / F::new(24.0) + t55114 + t55117 + t56366 / F::new(384.0) + t3066 * t2409 * t3067 * t4227 * t3306 / F::new(24.0) + t2408 * t2409 * t2376 * t1205 * t9807 / F::new(48.0) - F::new(5.0) / F::new(384.0) * t56374 + F::new(35.0) / F::new(216.0) * t52251 + t3066 * t2409 * t12213 * t15025 / F::new(24.0) - F::new(35.0) / F::new(216.0) * t55145 - t11342 * t4083 / F::new(96.0) - F::new(5.0) / F::new(384.0) * t56404 - t56431 / F::new(768.0);
    t58140
}
