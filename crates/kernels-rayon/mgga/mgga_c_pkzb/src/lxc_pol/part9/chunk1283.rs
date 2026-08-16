//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1283/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1283(t22181: f64, t22503: f64, t3139: f64, t22390: f64, t22393: f64, t22396: f64, t22398: f64, t22400: f64, t22404: f64, t22406: f64, t22408: f64, t22410: f64, t22478: f64, t22480: f64, t22482: f64, t22487: f64, t22490: f64, t22492: f64, t22494: f64, t22496: f64, t22499: f64, t22502: f64) -> (f64, f64) {
    let t22506 = 0.31168546390226634765e3_f64 * t22503 * t3139 * t22181;
    let t22507 = t22390 - t22393 - t22396 + t22398 + t22400 + t22404 + t22406 - t22408 - t22410 - t22478 - t22480 + t22482 - t22487 - t22490 - t22492 - t22494 + t22496 + t22499 + t22502 + t22506;
    (t22506, t22507)
}
