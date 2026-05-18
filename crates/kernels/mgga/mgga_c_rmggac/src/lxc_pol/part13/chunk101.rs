//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 101/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk101<F: Float>(t50: F, t73: F, t75: F, t80: F, t77: F, t8: F, t78: F, t76: F) -> (F, F, F, F, F, F, F, F) {
    let t294 = t73 * t50;
    let t295 = t75 * t294;
    let t296 = t295 * t80;
    let t297 = t77 * t8;
    let t298 = t78 * t297;
    let t299 = F::new(1.0) / t298;
    let t300 = t76 * t299;
    let t302 = -F::new(12.0) * t296 + F::new(12.0) * t300;
    (t294, t295, t296, t297, t298, t299, t300, t302)
}
