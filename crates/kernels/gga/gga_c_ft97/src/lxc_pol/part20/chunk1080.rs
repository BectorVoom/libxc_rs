//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1080/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1080<F: Float>(t108429: F, t1882: F, t27866: F, t27869: F, t108403: F, t108406: F, t108409: F, t108412: F, t108415: F, t108418: F, t108421: F, t108424: F, t108427: F, t108206: F, t446: F, t9770: F) -> (F, F, F, F) {
    let t108430 = 4.0 / 9.0 * t108429;
    let t108431 = t1882 * t27866;
    let t108432 = 4.0 / 9.0 * t108431;
    let t108433 = t1882 * t27869;
    let t108434 = 4.0 / 27.0 * t108433;
    let t108435 = -8.0 / 9.0 * t108403 - 2.0 / 3.0 * t108406 - 4.0 / 3.0 * t108409 + 8.0 / 3.0 * t108412 + 4.0 / 9.0 * t108415 - 2.0 / 3.0 * t108418 + 8.0 / 3.0 * t108421 - 4.0 / 3.0 * t108424 - 4.0 / 3.0 * t108427 + t108430 + t108432 - t108434;
    let t108437 = t446 * t9770 * t108206;
    (t108431, t108433, t108435, t108437)
}
