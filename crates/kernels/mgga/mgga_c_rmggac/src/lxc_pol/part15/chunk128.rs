//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 128/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk128<F: Float>(t167: F, t180: F, t249: F, t361: F, t380: F, t396: F, t403: F, t411: F, t418: F, t5: F) -> (F,) {
    let t421 = 0.53237641966666666666e-3 * t5 * t249 * t167 + 1.0 * t396 * t403 - t361 - t380 + 0.18311447306006545054e-3 * t5 * t249 * t180 + 0.5848223622634646207e0 * t411 * t418;
    (t421,)
}
