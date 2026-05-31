//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1061/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1061<F: Float>(t140: F, t1470: F, t21252: F, t21256: F, t2242: F, t27308: F, t27319: F, t27321: F, t27355: F, t31024: F, t31041: F, t31045: F, t31081: F, t31106: F, t31114: F, t31118: F, t31352: F, t31356: F, t31379: F, t31385: F, t31388: F, t4253: F, t476: F, t479: F, t6256: F, t6267: F, t6278: F, t7869: F, t7878: F) -> F {
    let t31392 = -F::cast_from(0.13265555555555555556e0_f64) * t1470 * t31352 - F::cast_from(0.11791604938271604938e0_f64) * t1470 * t31356 - F::cast_from(0.53062222222222222221e-1_f64) * t27308 - F::cast_from(0.88437037037037037035e-1_f64) * t27319 - F::cast_from(0.10612444444444444444e0_f64) * t27321 + F::cast_from(0.5572125e-1_f64) * t4253 * t31081 - F::cast_from(0.139303125e-1_f64) * t6256 * t31114 + F::cast_from(0.139303125e-1_f64) * t6256 * t31118 - F::cast_from(0.79593333333333333333e-1_f64) * t27355 - F::cast_from(0.27860625e-1_f64) * t4253 * t31045 - F::cast_from(0.232171875e-2_f64) * t476 * t31024 + F::cast_from(0.371475e-1_f64) * t2242 * t7878 + F::cast_from(0.371475e-1_f64) * t6267 * t31106 - F::cast_from(0.27860625e-1_f64) * t4253 * t31041 - F::cast_from(0.39796666666666666666e-1_f64) * t140 * t479 * t31379 - F::cast_from(0.5572125e-1_f64) * t21252 * t7869 + F::cast_from(0.15918666666666666666e0_f64) * t6278 * t31385 - F::cast_from(0.13265555555555555555e0_f64) * t6278 * t31388 + F::cast_from(0.26531111111111111111e-1_f64) * t21256;
    t31392
}
