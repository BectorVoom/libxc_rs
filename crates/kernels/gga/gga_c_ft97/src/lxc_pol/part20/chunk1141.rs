//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1141/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1141<F: Float>(t108429: F, t108431: F, t108433: F, t108403: F, t108406: F, t108409: F, t108412: F, t108415: F, t108418: F, t108421: F, t108424: F, t108427: F, t108437: F, t108441: F, t109325: F, t109329: F, t109333: F, t109337: F, t109341: F, t109345: F, t109350: F, t97084: F, t97089: F, t97092: F) -> (F, F) {
    let t110182 = 4.0 / 27.0 * t108429;
    let t110183 = 4.0 / 27.0 * t108431;
    let t110184 = 4.0 / 81.0 * t108433;
    let t110185 = -8.0 / 27.0 * t108403 - 2.0 / 9.0 * t108406 - 4.0 / 9.0 * t108409 + 8.0 / 9.0 * t108412 + 4.0 / 27.0 * t108415 - 2.0 / 9.0 * t108418 + 8.0 / 9.0 * t108421 - 4.0 / 9.0 * t108424 - 4.0 / 9.0 * t108427 + t110182 + t110183 - t110184;
    let t110198 = -4.0 / 9.0 * t108437 - 2.0 / 9.0 * t108441 - t109325 / 6.0 + 8.0 * t109329 + t97084 / 9.0 - 2.0 / 9.0 * t109333 + 4.0 / 3.0 * t109337 - t109341 / 9.0 - 2.0 / 9.0 * t97089 - 4.0 / 9.0 * t97092 + t109345 / 18.0 - t109350 / 6.0;
    (t110185, t110198)
}
