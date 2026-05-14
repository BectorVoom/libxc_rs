//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1324/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1324<F: Float>(t30420: F, t8392: F, t1882: F, t30512: F, t30314: F, t106875: F, t107024: F, t107041: F, t107043: F, t107059: F, t107068: F, t107077: F, t12968: F, t1391: F, t16919: F, t17384: F, t17388: F, t1901: F, t2142: F, t23571: F, t27011: F, t30489: F, t3446: F, t446: F, t47659: F, t574: F, t64242: F, t95837: F) -> (F,) {
    let t121227 = t8392 * t30420;
    let t121232 = t1882 * t30512;
    let t121241 = t1882 * t30314;
    let t121251 = -2.0 / 3.0 * t1901 * t12968 * t23571 * t17388 - 2.0 / 27.0 * t121227 - 4.0 / 3.0 * t1901 * t64242 * t27011 - t107024 - t107041 - t107043 + t107059 + 2.0 / 81.0 * t121232 + 2.0 / 3.0 * t446 * t574 * t2142 * t30489 - t107068 + 4.0 / 9.0 * t47659 * t95837 * t17384 + 2.0 / 9.0 * t121241 - 8.0 / 27.0 * t107077 - t446 * t574 * t1391 * t16919 / 3.0 + 2.0 / 9.0 * t1901 * t106875 * t3446;
    (t121251,)
}
