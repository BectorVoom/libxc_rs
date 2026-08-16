//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1329/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1329<F: Float>(t1306: F, t22480: F, t22482: F, t22487: F, t22490: F, t22492: F, t22494: F, t22496: F, t22499: F, t22502: F, t22506: F, t2457: F, t8568: F) -> F {
    let t23551 = -F::cast_from(3.0_f64) * t1306 * t2457 * t8568 - t22480 + t22482 - t22487 - t22490 - t22492 - t22494 + t22496 + t22499 + t22502 + t22506;
    t23551
}
