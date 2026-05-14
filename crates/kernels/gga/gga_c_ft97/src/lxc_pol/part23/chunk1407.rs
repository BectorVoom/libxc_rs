//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1407/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1407<F: Float>(t1882: F, t31820: F, t31885: F, t31847: F, t31799: F, t8392: F, t1091: F, t114554: F, t114770: F, t114772: F, t1255: F, t15294: F, t15369: F, t1901: F, t19367: F, t2347: F, t24898: F, t28501: F, t28506: F, t2862: F, t28760: F, t2881: F, t29051: F, t29056: F, t3281: F, t3746: F, t3886: F, t446: F, t69996: F, t7124: F, t7131: F, t72397: F, t72443: F, t835: F) -> (F,) {
    let t128371 = t1882 * t31820;
    let t128373 = t1882 * t31885;
    let t128387 = t1882 * t31847;
    let t128408 = t8392 * t31799;
    let t128413 = 2.0 / 3.0 * t128371 - 4.0 / 9.0 * t128373 + 4.0 / 9.0 * t3281 * t835 * t7131 * t3746 + t114770 + t114772 + 4.0 / 3.0 * t446 * t2862 * t1255 * t28506 + 4.0 / 3.0 * t446 * t2862 * t1255 * t28501 - 2.0 / 9.0 * t128387 - 2.0 / 3.0 * t1901 * t15369 * t24898 * t19367 - 4.0 / 3.0 * t1901 * t69996 * t29051 - 4.0 / 3.0 * t1901 * t72397 * t29056 + 2.0 / 9.0 * t1901 * t2881 * t114554 * t1091 + 4.0 / 27.0 * t1901 * t15294 * t7124 * t2347 * t3886 + 2.0 / 27.0 * t128408 - 4.0 / 9.0 * t1901 * t72443 * t28760;
    (t128413,)
}
