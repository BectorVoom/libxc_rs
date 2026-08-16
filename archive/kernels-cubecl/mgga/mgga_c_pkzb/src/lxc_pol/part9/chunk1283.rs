//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1283/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1283<F: Float>(t22181: F, t22503: F, t3139: F, t22390: F, t22393: F, t22396: F, t22398: F, t22400: F, t22404: F, t22406: F, t22408: F, t22410: F, t22478: F, t22480: F, t22482: F, t22487: F, t22490: F, t22492: F, t22494: F, t22496: F, t22499: F, t22502: F) -> (F, F) {
    let t22506 = F::cast_from(0.31168546390226634765e3_f64) * t22503 * t3139 * t22181;
    let t22507 = t22390 - t22393 - t22396 + t22398 + t22400 + t22404 + t22406 - t22408 - t22410 - t22478 - t22480 + t22482 - t22487 - t22490 - t22492 - t22494 + t22496 + t22499 + t22502 + t22506;
    (t22506, t22507)
}
