//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 438/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk438<F: Float>(t238: F, t218: F, t7203: F, t665: F, t7205: F, t1408: F, t1412: F, t1420: F, t6815: F, t7448: F, t7453: F, t7456: F, t7458: F, t7466: F, t7471: F, t7477: F) -> (F, F, F, F) {
    let t239 = 0.1e-59 < t238;
    let t7478 = t7203 * t218;
    let t7479 = t7205 * t665;
    let t7480 = t7478 * t7479;
    let t7484 = piecewise3(t239, 2.0 * t7448 - 0.88910709717637694816e-2 * t1412 * t1408 - 0.76612330055555555556e-1 * t7453 * t1420 + 0.22227677429409423704e-2 * t7456 * t7458 + 0.19762785756235085044e-4 * t238 * t7466 + 0.34058283191806748844e-3 * t6815 * t7471 - 0.22227677429409423704e-2 * t238 * t7458 + 0.58694491165413811142e-2 * t7477 * t7480, 0.0);
    (t7478, t7479, t7480, t7484)
}
