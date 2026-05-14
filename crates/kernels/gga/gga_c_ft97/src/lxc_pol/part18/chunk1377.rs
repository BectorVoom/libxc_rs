//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1377/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1377<F: Float>(t1882: F, t26876: F, t27008: F, t8392: F, t27400: F, t26830: F, t27307: F, t105705: F, t106204: F, t11593: F, t13044: F, t144: F, t167: F, t1901: F, t2142: F, t2157: F, t2185: F, t2210: F, t23463: F, t24078: F, t26768: F, t27263: F, t3052: F, t446: F, t569: F, t574: F, t605: F, t609: F, t6615: F, t925: F, t95751: F, t95797: F, t95799: F) -> (F,) {
    let t107022 = 2.0 / 9.0 * t1882 * t26876;
    let t107024 = 2.0 / 27.0 * t8392 * t27008;
    let t107041 = 4.0 / 9.0 * t1882 * t27400;
    let t107043 = 4.0 / 9.0 * t1882 * t26830;
    let t107059 = 2.0 / 9.0 * t1882 * t27307;
    let t107065 = -t107022 - t107024 - 2.0 / 9.0 * t1901 * t95751 * t13044 - t446 * t144 * t106204 / 3.0 - t446 * t569 * t24078 * t925 / 9.0 + t446 * t574 * t605 * t6615 * t2157 / 3.0 - t107041 - t107043 + 4.0 / 3.0 * t446 * t2185 * t167 * t105705 + 2.0 / 3.0 * t446 * t574 * t605 * t26768 * t609 + 4.0 / 9.0 * t95797 + 4.0 / 9.0 * t11593 * t2210 * t23463 * t3052 + t107059 + 2.0 / 9.0 * t95799 + 2.0 / 3.0 * t446 * t574 * t2142 * t27263;
    (t107065,)
}
