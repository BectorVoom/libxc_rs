//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1123/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1123<F: Float>(t24237: F, t28027: F, t28043: F, t1173: F, t6061: F, t1403: F, t2399: F, t6753: F, t10052: F, t1425: F, t14295: F, t193: F, t2354: F, t24191: F, t2459: F, t27956: F, t27963: F, t27986: F, t28467: F, t4003: F, t5996: F, t6002: F, t6008: F, t6752: F, t684: F, t713: F, t766: F, t96808: F, t96854: F, t96857: F) -> (F,) {
    let t109533 = 2.0 / 27.0 * t24237 * t28027;
    let t109535 = 2.0 / 27.0 * t24237 * t28043;
    let t109536 = t6061 * t1173;
    let t109556 = t1403 * t2399 * t6753;
    let t109561 = -t1403 * t193 * t96808 * t6752 / 3.0 - t1403 * t193 * t6008 * t1173 * t2459 / 3.0 - 2.0 / 3.0 * t1403 * t193 * t6008 * t4003 * t713 - t96854 / 9.0 - t96857 / 18.0 - t109533 - t109535 - t6002 * t2354 * t109536 * t684 / 9.0 - 2.0 / 3.0 * t1403 * t193 * t24191 * t27963 - 2.0 / 3.0 * t1403 * t193 * t24191 * t27956 + t1403 * t193 * t1425 * t14295 / 6.0 - 2.0 / 3.0 * t5996 * t28467 - 4.0 / 27.0 * t109556 - 24.0 * t10052 * t27986 * t766;
    (t109561,)
}
