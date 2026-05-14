//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 137/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk137<F: Float>(t361: F, t380: F, t383: F, t387: F, t390: F, t423: F, t425: F, t430: F, t435: F, t195: F) -> (F, F, F) {
    let t449 = t361 + t380 + t383 - t387 + t390 + t423 + t425 - t430 - t435;
    let t452 = t195 * t195;
    let t453 = 1.0 / t452;
    (t449, t452, t453)
}
