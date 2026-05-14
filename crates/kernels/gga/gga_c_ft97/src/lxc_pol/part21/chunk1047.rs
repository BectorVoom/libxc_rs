//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1047/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1047<F: Float>(t38477: F, t5502: F, t22914: F, t26119: F, t1307: F, t1882: F, t25969: F, t25885: F, t93506: F, t23054: F, t25872: F, t2: F, t25846: F, t23008: F, t25896: F, t458: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t100128 = t38477 * t5502;
    let t100147 = t22914 * t26119 / 27.0;
    let t100178 = t38477 * t1307;
    let t100252 = t1882 * t25969;
    let t100253 = 4.0 / 27.0 * t100252;
    let t100270 = t93506 * t25885;
    let t100271 = t100270 / 9.0;
    let t100272 = t23054 * t25872;
    let t100273 = 2.0 / 3.0 * t100272;
    let t100285 = t2 * t25846;
    let t100292 = t23008 * t458 * t25896;
    (t100128, t100147, t100178, t100252, t100253, t100270, t100271, t100272, t100273, t100285, t100292)
}
