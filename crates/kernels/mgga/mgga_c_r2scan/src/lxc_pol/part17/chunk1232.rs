//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1232/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1232<F: Float>(t37947: F, t37951: F, t39882: F, t43348: F, t43351: F, t43356: F, t43359: F, t43362: F, t43365: F, t43368: F, t43372: F, t41601: F, t41605: F, t43376: F, t43379: F, t43381: F, t43384: F, t43387: F, t43390: F, t43393: F, t43396: F, t43399: F, t43401: F) -> (F, F) {
    let t44359 = F::cast_from(0.1047928639570397803e0_f64) * t43348 + F::cast_from(0.20803732176130244552e1_f64) * t43351 + F::cast_from(0.31147743054556651237e-1_f64) * t37947 + F::cast_from(0.93443229163669953711e-1_f64) * t37951 - F::cast_from(0.95219938395347901947e-2_f64) * t43356 + F::cast_from(0.26198215989259945076e-1_f64) * t43359 - F::cast_from(0.57131963037208741167e-1_f64) * t43362 + F::cast_from(0.14282990759302185292e-1_f64) * t43365 - F::cast_from(0.13869154784086829701e1_f64) * t43368 + F::cast_from(0.93149212406257582492e-1_f64) * t43372 + F::cast_from(0.90044238659382329742e0_f64) * t39882;
    let t44370 = -t41601 + F::cast_from(0.26198215989259945076e-1_f64) * t43376 - F::cast_from(0.17465477326173296718e-1_f64) * t43379 - F::cast_from(0.26198215989259945076e-1_f64) * t43381 - F::cast_from(0.26198215989259945076e-1_f64) * t43384 - F::cast_from(0.26198215989259945076e-1_f64) * t43387 + F::cast_from(0.52396431978519890152e-1_f64) * t43390 + F::cast_from(0.26198215989259945076e0_f64) * t43393 - F::cast_from(0.1047928639570397803e0_f64) * t43396 - F::cast_from(0.13099107994629972538e-1_f64) * t43399 - F::cast_from(0.19514881078765566037e-1_f64) * t43401 - t41605;
    (t44359, t44370)
}
