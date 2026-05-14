//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1402/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1402<F: Float>(t113579: F, t114452: F, t114453: F, t114454: F, t127007: F, t127010: F, t127015: F, t127019: F, t127024: F, t127027: F, t127032: F, t127037: F, t113591: F, t114465: F, t114467: F, t114469: F, t127042: F, t127731: F, t127735: F, t127739: F, t127742: F, t127744: F, t127748: F, t127752: F) -> (F, F) {
    let t128269 = 2.0 / 3.0 * t127007 + t114452 + t114453 - t114454 + t127010 / 9.0 + t127015 / 9.0 - 2.0 * t127019 + 4.0 / 81.0 * t113579 - t127024 / 3.0 - 2.0 * t127027 + t127032 / 18.0 + 2.0 / 3.0 * t127037;
    let t128279 = 2.0 / 3.0 * t127042 - t127731 / 3.0 + 16.0 / 27.0 * t113591 - t114465 - t114467 - t114469 + 2.0 / 3.0 * t127735 + 2.0 / 3.0 * t127739 - 4.0 / 9.0 * t127742 - 2.0 / 81.0 * t127744 - t127748 / 6.0 - 4.0 * t127752;
    (t128269, t128279)
}
