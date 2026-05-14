//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 968/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk968<F: Float>(t27028: F, t27051: F, t27110: F, t30176: F, t30180: F, t30184: F, t30189: F, t30193: F, t30197: F, t30201: F, t30205: F, t30209: F, t30214: F, t30221: F, t30225: F, t30229: F) -> (F,) {
    let t30231 = t30176 / 9.0 + t30180 / 18.0 + t30184 / 27.0 - t30189 / 8.0 - t30193 / 6.0 + t30197 / 9.0 + 2.0 / 27.0 * t30201 + 2.0 / 9.0 * t30205 - t30209 / 9.0 - t30214 / 3.0 + t27028 / 9.0 - 2.0 / 27.0 * t27051 - 2.0 / 9.0 * t27110 - t30221 / 18.0 - 2.0 / 9.0 * t30225 - 2.0 / 9.0 * t30229;
    (t30231,)
}
