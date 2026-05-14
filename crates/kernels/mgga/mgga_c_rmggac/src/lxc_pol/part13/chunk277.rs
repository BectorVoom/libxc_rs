//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 277/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk277<F: Float>(t60: F, t441: F, t50: F, t1383: F, t284: F, t814: F, t1382: F, zeta_threshold: F) -> (F, F) {
    let t61 = t60 <= zeta_threshold;
    let t1386 = t441 * t50;
    let t1390 = piecewise3(t61, 0.0, -2.0 / 9.0 * t1383 * t284 - 4.0 / 3.0 * t1386 * t814);
    let t1392 = t1382 / 2.0 + t1390 / 2.0;
    (t1386, t1392)
}
