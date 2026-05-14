//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 264/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk264<F: Float>(t1007: F, t78: F, t76: F, t1281: F, t1283: F) -> (F, F, F) {
    let t1284 = t78 * t1007;
    let t1285 = 1.0 / t1284;
    let t1287 = 156.0 * t76 * t1285;
    let t1288 = -t1281 + t1283 - t1287;
    (t1285, t1287, t1288)
}
