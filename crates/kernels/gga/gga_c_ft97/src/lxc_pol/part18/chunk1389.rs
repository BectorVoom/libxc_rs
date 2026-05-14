//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1389/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1389<F: Float>(t6641: F, t8232: F, t26932: F, t8392: F, t1882: F, t27316: F, t6720: F, t27249: F, t26985: F, t104453: F, t12602: F, t12664: F, t13013: F, t13017: F, t13140: F, t13161: F, t144: F, t1901: F, t1986: F, t2185: F, t23470: F, t23495: F, t26863: F, t27015: F, t3435: F, t446: F, t574: F, t605: F, t6718: F, t95767: F, t96232: F) -> (F,) {
    let t107563 = t8232 * t6641;
    let t107566 = 2.0 / 27.0 * t8392 * t26932;
    let t107573 = 2.0 / 9.0 * t1882 * t27316;
    let t107574 = t8232 * t6720;
    let t107589 = 2.0 / 27.0 * t8392 * t27249;
    let t107603 = 2.0 / 27.0 * t8392 * t26985;
    let t107604 = 4.0 / 27.0 * t107563 - t107566 + 2.0 / 9.0 * t96232 - 4.0 / 3.0 * t1901 * t13140 * t27015 * t12602 + t107573 - 4.0 / 27.0 * t107574 - 2.0 / 3.0 * t446 * t574 * t12664 * t23495 - 2.0 * t446 * t144 * t104453 - 2.0 / 3.0 * t446 * t2185 * t605 * t6718 * t1986 - t107589 + 2.0 / 9.0 * t1901 * t23470 * t13161 + t1901 * t23470 * t13013 / 9.0 + 2.0 / 27.0 * t1901 * t26863 * t13017 + 4.0 / 9.0 * t1901 * t95767 * t3435 - t107603;
    (t107604,)
}
