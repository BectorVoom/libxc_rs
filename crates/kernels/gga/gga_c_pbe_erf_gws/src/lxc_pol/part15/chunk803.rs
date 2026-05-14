//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 803/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk803<F: Float>(t1017: F, t7514: F, t1828: F, t7062: F, t5349: F, t7461: F, t7466: F, t7472: F, t7474: F, t7476: F, t7478: F, t7479: F, t7480: F, t7482: F, t7489: F, t7494: F, t7498: F, t7504: F, t7509: F, t7513: F) -> (F, F, F) {
    let t7515 = t7514 * t1017;
    let t7516 = t7515 * t1828;
    let t7518 = 16.0 / 45.0 * t7062 * t7516;
    let t7519 = 8.0 / 45.0 * t5349;
    let t7520 = -t7461 + t7466 - t7472 - t7474 - t7476 - t7478 + t7479 + t7480 + t7482 - t7489 + t7494 - t7498 - t7504 + t7509 - t7513 + t7518 + t7519;
    (t7518, t7519, t7520)
}
