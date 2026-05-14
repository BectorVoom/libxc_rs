//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 935/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk935<F: Float>(t21239: F, t4271: F, t7706: F, t14496: F, t14497: F, t30153: F, t30875: F, t416: F, t30273: F, t6287: F, t30294: F, t6279: F, t140: F, t1470: F, t21252: F, t21256: F, t2242: F, t27308: F, t27319: F, t27321: F, t27355: F, t31024: F, t31041: F, t31045: F, t31081: F, t31106: F, t31114: F, t31118: F, t4253: F, t476: F, t479: F, t6256: F, t6267: F, t6278: F, t7869: F, t7878: F) -> (F,) {
    let t31352 = t4271 * t21239 * t7706;
    let t31356 = t14496 * t14497 * t30153;
    let t31379 = t416 * t30875;
    let t31385 = t6287 * t30273;
    let t31388 = t6279 * t30294;
    let t31392 = -0.13265555555555555556e0 * t1470 * t31352 - 0.11791604938271604938e0 * t1470 * t31356 - 0.53062222222222222221e-1 * t27308 - 0.88437037037037037035e-1 * t27319 - 0.10612444444444444444e0 * t27321 + 0.5572125e-1 * t4253 * t31081 - 0.139303125e-1 * t6256 * t31114 + 0.139303125e-1 * t6256 * t31118 - 0.79593333333333333333e-1 * t27355 - 0.27860625e-1 * t4253 * t31045 - 0.232171875e-2 * t476 * t31024 + 0.371475e-1 * t2242 * t7878 + 0.371475e-1 * t6267 * t31106 - 0.27860625e-1 * t4253 * t31041 - 0.39796666666666666666e-1 * t140 * t479 * t31379 - 0.5572125e-1 * t21252 * t7869 + 0.15918666666666666666e0 * t6278 * t31385 - 0.13265555555555555555e0 * t6278 * t31388 + 0.26531111111111111111e-1 * t21256;
    (t31392,)
}
