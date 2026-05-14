//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1382/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1382<F: Float>(t27239: F, t8392: F, t1882: F, t26973: F, t26851: F, t27246: F, t158: F, t23455: F, t104465: F, t104498: F, t104529: F, t104586: F, t12599: F, t12715: F, t13140: F, t144: F, t1901: F, t2142: F, t23478: F, t26883: F, t27015: F, t3420: F, t3478: F, t446: F, t47666: F, t574: F, t6639: F, t9428: F, t95767: F, t95975: F) -> (F, F) {
    let t107234 = 4.0 / 27.0 * t8392 * t27239;
    let t107236 = 2.0 / 9.0 * t1882 * t26973;
    let t107241 = 2.0 / 9.0 * t1882 * t26851;
    let t107243 = 2.0 / 27.0 * t8392 * t27246;
    let t107273 = t158 * t23455;
    let t107277 = t107234 + t107236 + 2.0 / 9.0 * t1901 * t95767 * t3420 - t107241 - t107243 - 8.0 / 81.0 * t95975 + 4.0 / 3.0 * t446 * t144 * t104586 + t446 * t574 * t9428 * t6639 / 3.0 + 2.0 / 3.0 * t446 * t574 * t2142 * t26883 - 2.0 / 3.0 * t446 * t144 * t104465 - t446 * t144 * t104498 / 3.0 + 4.0 / 3.0 * t446 * t144 * t104529 - 2.0 / 3.0 * t1901 * t13140 * t27015 * t12599 + 2.0 / 3.0 * t446 * t574 * t23478 * t3478 - 8.0 / 27.0 * t47666 * t107273 * t12715;
    (t107273, t107277)
}
