//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1128/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1128<F: Float>(t1403: F, t2399: F, t6839: F, t6837: F, t771: F, t6749: F, t96360: F, t1424: F, t4003: F, t24429: F, t3972: F, t1131: F, t14361: F, t1454: F, t193: F, t2354: F, t2405: F, t2409: F, t2413: F, t2617: F, t27908: F, t27991: F, t28026: F, t28461: F, t5996: F, t6002: F, t6008: F, t684: F, t719: F, t96798: F, t9744: F) -> (F, F) {
    let t109711 = t1403 * t2399 * t6839;
    let t109713 = t6837 * t771;
    let t109731 = t96360 * t6749;
    let t109735 = t1424 * t4003;
    let t109747 = t24429 * t3972;
    let t109749 = 2.0 / 9.0 * t6002 * t96798 * t28026 + 2.0 / 27.0 * t109711 - t6002 * t2354 * t109713 * t684 / 9.0 - t6002 * t2354 * t27991 * t2413 / 18.0 - t6002 * t9744 * t27991 * t2405 / 27.0 - t1403 * t193 * t6008 * t2617 * t1131 / 3.0 - 2.0 / 81.0 * t109731 + t5996 * t27908 / 3.0 - t6002 * t2354 * t109735 * t684 / 9.0 - 2.0 * t719 * t28461 + t6002 * t2354 * t27991 * t2409 / 9.0 - t14361 * t1454 - 4.0 * t109747;
    (t109747, t109749)
}
