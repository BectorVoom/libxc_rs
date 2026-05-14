//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1316/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1316<F: Float>(t105733: F, t106087: F, t106088: F, t106093: F, t106095: F, t106097: F, t106098: F, t119968: F, t119970: F, t95225: F, t95228: F, t95242: F, t106115: F, t106118: F, t119978: F, t119982: F, t119984: F, t119988: F, t119992: F, t119996: F, t120000: F, t120004: F, t120006: F) -> (F, F) {
    let t121016 = -8.0 / 27.0 * t105733 + 4.0 / 27.0 * t95225 + t95228 / 27.0 - t106087 - t106088 - t106093 + t106095 - t106097 + t106098 - 2.0 / 9.0 * t119968 + 4.0 / 27.0 * t119970 - 4.0 / 27.0 * t95242;
    let t121028 = t106115 + t119978 / 4.0 - t119982 / 18.0 - 2.0 / 27.0 * t119984 + 2.0 / 9.0 * t119988 + t119992 / 9.0 + 2.0 / 27.0 * t119996 - 4.0 * t120000 - t120004 / 6.0 + 2.0 / 27.0 * t120006 + t106118;
    (t121016, t121028)
}
