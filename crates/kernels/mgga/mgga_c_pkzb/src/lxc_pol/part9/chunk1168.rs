//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1168/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1168<F: Float>(t22474: F, t22162: F, t22164: F, t22167: F, t22169: F, t22171: F, t22175: F, t22184: F, t22188: F, t22313: F, t22355: F, t22359: F, t22361: F, t22363: F, t22366: F, t22374: F, t22376: F, t22378: F, t22380: F, t22382: F, t22385: F) -> (F, F) {
    let t22475 = 0.28582678745379824648e-3 * t22474;
    let t22476 = t22162 + t22164 + t22167 - t22169 + t22171 + t22175 - t22184 - t22188 - t22313 + t22355 + t22359 + t22361 - t22363 - t22366 - t22374 - t22376 - t22378 + t22380 + t22382 - t22385;
    (t22475, t22476)
}
